//
//  main.cpp
//  deeznuts
//
//  Created by Joey Buiteweg on 2/24/16.
//  Copyright © 2016 Joey Buiteweg. All rights reserved.
//

//#include <iostream>
//using namespace std;
//
//
//int main(int argc, const char * argv[]) {
//    string what;
//    cout << "man nigga fuck yo couch" << endl;
//    getline(cin, what);
//    cout << "deez " << what << " on your chin \n";
//    cout << argv;
//}

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

 