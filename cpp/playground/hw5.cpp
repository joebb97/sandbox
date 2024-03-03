#include <iostream>
#include <string>
#include <vector>
#include "hw2.h"

using namespace std;

void printVec(vector<string> mom)
{
    for (int i = 0; i < mom.size(); i++)
        cout << mom[i] << endl;
}

int main()
{
    vector<string> victor;
    
    victor.push_back("lolol");
    for (int i = 0; i < 10; i++)
        victor.push_back("idk");
    
    vector< vector<int> > twod;
    vector<int> yaaas;
    
    for (int i = 0; i < 5; i++)
        yaaas.push_back(i);
    
    twod.push_back(yaaas);
    twod.push_back(yaaas);
    
    for (int i = 0; i < twod.size(); i++)
    {
        for (int ja = 0; ja < twod[i].size(); ja++)
        {
            cout << twod[i][ja] << "\t";
        }
        cout << "\n";
    }
    cout << "\n";
    
    //printVec(victor);
}

 
