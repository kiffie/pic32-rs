#!/bin/bash

build_module() {

    local edc_url=$1
    local module=$2

    local edc=$(mktemp /tmp/${module}_XXXXXX.edc)
    echo $edc
    curl $edc_url > $edc
    ./pic2config.py -o "src/$module.rs" $edc || exit -1
}

URLBASE=https://raw.githubusercontent.com/kiffie/pic32-pac/master/pic32mx2xx
URLBASE_MX567=https://raw.githubusercontent.com/kiffie/pic32-pac/mx567

build_module $URLBASE/pic32mx1xxfxxxb/PIC32MX170F256B.PIC pic32mx1xx
build_module $URLBASE/pic32mx2xxfxxxb/PIC32MX270F256B.PIC pic32mx2xx
build_module $URLBASE/pic32mx1x4fxxxb/PIC32MX174F256B.PIC pic32mx1x4
build_module $URLBASE/pic32mx2x4fxxxb/PIC32MX274F256B.PIC pic32mx2x4

build_module $URLBASE_MX567/pic32mx567/pic32mx695fxxxl/PIC32MX695F512L.PIC pic32mx567

build_module file:$(pwd)/PIC32MZ2048EFM144.PIC pic32mzef
