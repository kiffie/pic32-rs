#!/usr/bin/python3
#
# Copyright (c) 2021 Stephan <kiffie@mailbox.org>
#
# SPDX-License-Identifier: MIT OR Apache-2.0

"""
pic2config.py
Create a PIC32 configuration SFRS crate from an .PIC XML file
"""

import argparse
import xml.etree.ElementTree as ET
import sys

class Field:
    def __init__(self, name, pos, length, desc):
        self.name = name
        self.pos = pos
        self.length = length
        self.desc = desc

    def __repr__(self):
        return f'Field({self.name}, {self.pos}, {self.length}, {self.desc})'

class ConfigSFR:
    def __init__(self, e):
        self.name = e.get('{http://crownking/edc}cname')
        self.addr = e.get('{http://crownking/edc}_addr')
        self.default = e.get('{http://crownking/edc}default')

        ns = {'edc': 'http://crownking/edc'}
        self.fields = []
        pos = 0
        for f in e.findall('.//edc:DCRMode[@{http://crownking/edc}id="DS.0"]/*', ns):
            if f.tag == '{http://crownking/edc}DCRFieldDef':
                cname = f.get('{http://crownking/edc}cname')
                length = int(f.get('{http://crownking/edc}nzwidth'), 0)
                desc = f.get('{http://crownking/edc}desc')
                ishidden = f.get('{http://crownking/edc}ishidden') # TODO: check value
                if not ishidden:
                    self.fields.append(Field(cname, pos, length, desc))
                pos += length
            elif f.tag == '{http://crownking/edc}AdjustPoint':
                offset = int(f.get('{http://crownking/edc}offset'), 0)
                pos += offset
            else:
                raise(Exception(f"unexpected element {f.tag}"))

    def __str__(self):
        s = (f'ConfigSFR {self.name} at {self.addr}\n');
        for f in self.fields:
            s += (f'    {f}\n')
        return s


# patch input xml tree (== edc file)
def patch_edc(xml):
    ns = {'edc': 'http://crownking/edc'}
    device_name = xml.getroot().get('{http://crownking/edc}name')

    print(f'device_name = {device_name}')

    # correct default value for DEVCFG0 for PIC32MX1x4 and PIC32MX1x4
    if device_name.startswith('PIC32MX174') or device_name.startswith('PIC32MX274'):
        for e in xml.getroot().findall('.//edc:ConfigFuseSector/edc:DCRDef', ns):
            if e.get('{http://crownking/edc}cname') == 'DEVCFG0':
                e.set('{http://crownking/edc}default', '0xffffffff')

    # remove duplicate field semantic definitions for PIC32MZEF
    if device_name.startswith('PIC32MZ2048EF'):
        for fd in xml.getroot().findall('.//edc:ConfigFuseSector//edc:DCRFieldDef', ns):
            for fs in fd.findall('edc:DCRFieldSemantic', ns):
                fname = fd.get('{http://crownking/edc}cname')
                sname = fs.get('{http://crownking/edc}cname')
                if (fname == 'SOSCGAIN' or fname == 'POSCGAIN') and sname.endswith('X'):
                    fd.remove(fs)

def parseargs():
    parser = argparse.ArgumentParser()
    parser.add_argument("pic", help="Path to EDC/PIC file to load")
    parser.add_argument("-o", "--output", help="path to output file")
    args = parser.parse_args()
    return args


def main():
    args = parseargs()
    pic = args.pic

    xml = ET.parse(pic)

    if args.output:
        out = open(args.output, 'w')
    else:
        out = sys.stdout

    # patch input data
    patch_edc(xml)

    # find first ConfigFuseSector
    ns = {'edc': 'http://crownking/edc'}
    cfs = xml.getroot().find('.//edc:ConfigFuseSector', ns)

    beginaddr = int(cfs.get('{http://crownking/edc}beginaddr'), 0)
    endaddr = int(cfs.get('{http://crownking/edc}endaddr'), 0)
    n_words = (endaddr - beginaddr) // 4

    print('#![allow(non_snake_case)]', file=out)
    print('#![allow(non_camel_case_types)]\n', file=out)
    print('#![allow(clippy::upper_case_acronyms)]', file=out)
    print( '/// Length of config word sector in words', file=out)
    print(f'pub const CONFIG_SECTOR_LENGTH: usize = {n_words};\n', file=out)

    for i in cfs.findall('.//edc:DCRFieldDef', ns):
        cname = i.get('{http://crownking/edc}cname')
        desc = i.get('{http://crownking/edc}desc')
        ishidden = i.get('{http://crownking/edc}ishidden') # TODO: check value
        if not ishidden:
            if desc:
                print(f"/// {desc}", file=out)
            fsem = i.findall('./edc:DCRFieldSemantic', ns)
            if len(fsem) == 0: # field semantic information
                length = int(i.get('{http://crownking/edc}nzwidth'), 0)
                if length <= 16:
                    print(f'type {cname} = u16;\n', file=out)
                else:
                    print(f'type {cname} = u32;\n', file=out)
            else:
                print(f"pub enum {cname} {{", file=out)
                for e in fsem:
                    scname = e.get('{http://crownking/edc}cname')
                    desc = e.get('{http://crownking/edc}desc')
                    when = e.get('{http://crownking/edc}when')
                    val = when.rsplit("=", maxsplit=1)[-1].strip()
                    print(f"    /// {desc}", file=out)
                    print(f"    {scname} = {val},\n", file=out)
                print("}\n", file=out)

    sfrs = []
    for e in cfs.findall('.//edc:DCRDef', ns):
        sfr = ConfigSFR(e)
        sfrs.append(sfr)
        print(sfr)

    print('/// Configuration word sector', file=out)
    print('#[repr(C)]', file=out)
    print('pub struct ConfigSector {', file=out)
    for sfr in sfrs:
        print(f'    {sfr.name}: u32,', file=out)
    print('}\n', file=out)

    print('impl ConfigSector {', file=out)
    print('/// Create a builder', file=out)
    print('    pub const fn default() -> ConfigSectorBuilder {', file=out)
    print('        ConfigSectorBuilder {', file=out)
    for sfr in sfrs:
        print(f'            {sfr.name}: {sfr.default},', file=out)
    print( '        }', file=out)
    print( '    }\n', file=out)

    print('/// Convert into a array of 32 bit words consuming this ConfigSector', file=out)
    print(f'    pub const fn into_array(self) -> [u32; CONFIG_SECTOR_LENGTH] {{', file=out)
    print( '        [', end='', file=out)
    for sfr in sfrs:
        print(f'self.{sfr.name}, ', end='', file=out)
    print(']\n    }\n', file=out)
    print('}', file=out)

    print('/// Configuration word sector builder', file=out)
    print('pub struct ConfigSectorBuilder {', file=out)
    for sfr in sfrs:
        print(f'    {sfr.name}: u32,', file=out)
    print('}\n', file=out)

    print('impl ConfigSectorBuilder {', file=out)
    for sfr in sfrs:
        for field in sfr.fields:
            mask = ((1 << field.length) - 1) << field.pos
            print(f'    pub const fn {field.name}(mut self, v: {field.name}) -> Self {{', file=out)
            print(f'        self.{sfr.name} &= !{mask:#010x};', file=out)
            if field.pos > 0:
                print(f'        self.{sfr.name} |= (v as u32) << {field.pos};', file=out)
            else:
                print(f'        self.{sfr.name} |= v as u32;', file=out)
            print( '        self', file=out)
            print( '    }\n', file=out)

    print('    pub const fn build(self) -> ConfigSector {', file=out)
    print('        ConfigSector {', file=out)
    for sfr in sfrs:
        print(f'            {sfr.name}: self.{sfr.name},', file=out)
    print('        }', file=out)
    print('    }', file=out)
    print('}', file=out)

if __name__ == "__main__":
    main()
