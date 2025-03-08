echo "##### Compiling RUST API... #####"
cd api
cargo build --release

echo "##### Compiling C++... #####"
cd ..

if [ ! -d "./build" ]; then
    mkdir build
fi

cd build
cmake ../src
make

echo "##### Running target... #####"
echo
./main