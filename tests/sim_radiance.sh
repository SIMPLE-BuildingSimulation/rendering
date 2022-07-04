#!/usr/bin/bash


IMG=img.hdr
RPICT_OPTIONS="-ab 4 -aa 0.0"
RTRACE_OPTIONS="-lw 1e-10 -ad 10024 -aa 0"
POINTS="../../points.pts"
N_BOUNCES=12
WHITE_SKY=sky.rad
OCTREE=octree.oct
BLACK_OCTREE=black_octree.oct

# Run rtrace sims
# cd ray_tracer
# for dir in $(ls -d */)
# do 
#     cd $dir    
#     for rad in $(ls | grep .rad)
#     do        
#         echo Running sim on $dir
        
#         oconv -f $rad > $OCTREE # Frozen octree

#         # rpict $RPICT_OPTIONS -vp 2 1 1 -vd 0 1 0 -vh 60 -vv 60 -x 512 -y 512 $OCTREE > $IMG
#         cat $POINTS | rtrace -h -ab 0 $RTRACE_OPTIONS $OCTREE | rcalc -e '$1=$1*0.265 + $2*0.67 + $3*0.065' > direct_results.txt
#         cat $POINTS | rtrace -h -ab $N_BOUNCES $RTRACE_OPTIONS $OCTREE | rcalc -e '$1=$1*0.265 + $2*0.67 + $3*0.065' > global_results.txt
#         # echo 2 1 1 0 1 0 | rtrace -otopnv -h $RTRACE_OPTIONS $OCTREE  > results.txt

#         # echo 2 1 1 0 1 0 | /Users/germolinal/Documents/Radiance/build/UILD_HEADLESS/bin/Debug/rtrace -h /Users/germolinal/Documents/simple/rendering/tests/metal_box_diffuse/octree.oct

#         rm -rf $OCTREE
        
        
#     done
    
#     cd ..
# done
# cd ..


# DC sims
cd dc
for dir in $(ls -d */)
do 
    cd $dir    
    for rad in $(ls | grep .rad)
    do      
        
        # Build scene... for SIMPLE to read afterwards
        # xform ./room.rad ./windows.rad > ./scene.rad

        oconv -f ./room.rad ./windows.rad > $OCTREE # Frozen octree  


        echo "void plastic black 0 0 5 0 0 0 0 0" > aux
        echo "!xform -m black ./room.rad" >> aux
        echo "!xform ./windows.rad" >> aux
        oconv -f aux > $BLACK_OCTREE # Frozen octree  

        echo "#@rfluxmtx u=+Y h=u
            void glow groundglow
            0
            0
            4 1 1 1 0

            groundglow source ground
            0
            0
            4 0 0 -1 180

            #@rfluxmtx u=+Y h=r1
            void glow skyglow
            0
            0
            4 1 1 1 0

            skyglow source skydome
            0
            0
            4 0 0 1 180" > $WHITE_SKY
        
        
        N_SENSORS=14 
        # | tail -n $N_SENSORS 
        cat $POINTS | rfluxmtx -y $N_SENSORS -I+ -ab $N_BOUNCES $RTRACE_OPTIONS - $WHITE_SKY -i $BLACK_OCTREE   > direct_results.txt 
        cat $POINTS | rfluxmtx -y $N_SENSORS -I+ -ab $N_BOUNCES $RTRACE_OPTIONS - $WHITE_SKY -i $OCTREE  > global_results.txt 
        

        # Clean up
        rm $WHITE_SKY
        rm -rf $OCTREE
        rm aux
        rm $BLACK_OCTREE
    done
    
    cd ..
done

cd ..
