set +e

./configure
export GTK_VERSION=${GTK_VERSION:=$(pkg-config --modversion gtk+-3.0 | awk -F. '{print "GTK_3_"$2}')}
echo $GTK_VERSION
make glue
