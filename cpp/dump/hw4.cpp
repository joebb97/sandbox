#include <iostream>
#include <vector>
using namespace std;
void Experiment1(){
    vector<int> data;
    cout << *std::max_element(data.begin(), data.end()) << endl;
}


int main() {
    Experiment1();
    return 0;
}
