#!/bin/bash

VIEW="-vp 0 -13 0 -vd 0 1 0 -vh 60 -vv 60"
FILES=$(ls | grep .rad) 
AMBIENTS=( 2 )


OCTREE=octree.oct

for file in $(ls | grep .rad) #"${FILES[@]}"
do
    oconv $file > $OCTREE
    for ab in "${AMBIENTS[@]}"
    do
        OPTIONS="-ab $ab"        
        IMAGE="images/radiance_${file}_ab-$ab"
        rpict $OPTIONS $VIEW $OCTREE > "$IMAGE.hdr"
        falsecolor -i "$IMAGE.hdr" -s a > "${IMAGE}_fc.hdr"
        #pcond -h "$IMAGE.hdr" "${IMAGE}_humancond.hdr"

    done # end ambients
    rm $OCTREE
done # end files
