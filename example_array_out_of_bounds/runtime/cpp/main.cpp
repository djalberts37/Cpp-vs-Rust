#include <iostream>

int main(int argc, char *argv[])
{
    int array[10] = {0, 1, 2, 3, 4, 5, 6, 7, 8, 9};
    
    for(int i = 0; i < 11; i++)
    {
        std::cout << "Array[" << i << "] = " << array[i] << std::endl;
    }
}

