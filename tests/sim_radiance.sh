#!/usr/bin/bash

OCTREE=octree.oct
IMG=img.hdr
RPICT_OPTIONS="-ab 4 -aa 0.0"
RTRACE_OPTIONS="-lw 1e-10 -ad 10024 -aa 0"

cd ray_tracer
for dir in $(ls -d */)
do 
    cd $dir    
    for rad in $(ls | grep .rad)
    do        
        echo Running sim on $dir
        
        oconv -f $rad > $OCTREE # Frozen octree

        # rpict $RPICT_OPTIONS -vp 2 1 1 -vd 0 1 0 -vh 60 -vv 60 -x 512 -y 512 $OCTREE > $IMG
        cat ../../points.pts | rtrace -h -ab 0 $RTRACE_OPTIONS $OCTREE | rcalc -e '$1=$1*0.265 + $2*0.67 + $3*0.065' > direct_results.txt
        cat ../../points.pts | rtrace -h -ab 12 $RTRACE_OPTIONS $OCTREE | rcalc -e '$1=$1*0.265 + $2*0.67 + $3*0.065' > global_results.txt
        # echo 2 1 1 0 1 0 | rtrace -otopnv -h $RTRACE_OPTIONS $OCTREE  > results.txt

        # echo 2 1 1 0 1 0 | /Users/germolinal/Documents/Radiance/build/UILD_HEADLESS/bin/Debug/rtrace -h /Users/germolinal/Documents/simple/rendering/tests/metal_box_diffuse/octree.oct

        rm -rf $OCTREE
        
        
    done
    
    cd ..
done
cd ..