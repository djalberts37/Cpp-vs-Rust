#include <iostream>
#include <thread>

int main(int argc, char *argv[])
{
    int shared_value = 0;

    std::thread thread1([&]()
    {
        for(int i = 0; i < 1000000; i++)
        {
            shared_value++;
        }
    });

    std::thread thread2([&]()
    {
        for(int i = 0; i < 1000000; i++)
        {
            shared_value++;
        }
    });

    thread1.join();
    thread2.join();

    std::cout << "Shared_value = " << shared_value << std::endl;
}


