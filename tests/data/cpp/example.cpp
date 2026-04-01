#include <iostream>

int globalVar = 10;
class Car {
public:
    std::string brand;
    std::string model;
    int year;
    void display_details() {
        std::cout << "Brand: " << brand << ", Model: " << model << ", Year: " << year << std::endl;
    }
    void display_details2() {
        return;
    }
};

int multiply(int a, int b) {
    return a * b;
}
#define LONG_STRING "This is a very long string that " \
                    "spans multiple lines using " \
                    "backslashes for continuation."