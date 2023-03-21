#include <iostream>

int main(int argc, char *argv[])
{
    int* ptr = new int(42);

    delete ptr;

    int value = *ptr;
    std::cout << "Value = " << value << std::endl;
}


