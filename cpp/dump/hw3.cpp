//
//  main.cpp
//  cppmessaround
//
//  Created by Joey Buiteweg on 1/17/17.
//  Copyright © 2017 Joey Buiteweg. All rights reserved.
//

#include <iostream>
using namespace std;

void Experiment1(){
    string bigger  = "eamplex";
    string smaller = "eample";
    int mistakes_allowed = 1;
    int change_index = 0;
    int smaller_iterator = 0;
    for (int i = 0; i < int(bigger.size()); ++i){
        if (mistakes_allowed < 0){
            cout << "Not similar";
            return;
        }
        if (smaller[smaller_iterator] != bigger[i]){
            change_index = i;
            --mistakes_allowed;
            continue;
        }
        ++smaller_iterator;
    }
    if (mistakes_allowed == 1){
        change_index = int(bigger.size());
    }
    cout << "delete at " << change_index << endl;

    
}

void Experiment2(){
    string cand;
    string pred;
    cin >> cand >> pred;
    int difference = static_cast<int>(cand.size()) - static_cast<int>(pred.size());
    if (difference != 1 && difference != -1) {
        cout << "not similar" << endl;
        return;
    }
    bool insert;
    string bigger;
    string smaller;
    if (difference == 1){
        bigger = cand;
        smaller = pred;
        insert = true;
    }
    else {
        bigger = pred;
        smaller = cand;
        insert = false;
    }
    int differences = 0;
    size_t change_index = 0;
    for (size_t i = 0; i < smaller.size(); ++i){
        if (smaller[i] != bigger[i + differences]){
            if (smaller[i] != bigger[i + 1]){
                cout << "not similar" << endl;
                return;
            }
            change_index = i;
            ++differences;
        }
        
        if (differences > 1){
            cout << "not similar" << endl;
            return;
        }
    }
    if (differences == 0){
        change_index = bigger.size() - 1;
    }
    if (insert) cout << "insert " << bigger[change_index] << " at " << change_index << endl;
    else cout << "delete at " << change_index << endl;
}

void Experiment3() {
    string cand;
    string pred;
    cin >> cand;
    cin >> pred;
    if ((pred.size() != cand.size()) || (pred.size() == 1 && cand.size() == 1)){
        cout << "not similar " << endl;
        return;
    }
    int diffs = 0;
    size_t change_index = 0;
    bool similar_decided = false;
    bool similar = false;
    for (size_t i = 0; i < pred.size(); ++i){ //abc bac
        if (pred[i] != cand[i]){
            ++diffs;
        }
        if (diffs == 1 && !similar_decided){
            if (i == pred.size() - 1){
                break;
            }
            similar = (pred[i + 1] == cand[i]) && (cand[i + 1] == pred[i]);
            if (similar == false){
                cout << "not similar" << endl;
                return;
            }
            change_index = i;
            similar_decided = true;
            ++i;
        }
        
        if (diffs > 1){
            cout << "not similar" << endl;
            return;
        }
    }
    if (similar) cout << "swap at " << change_index << endl;
    else cout << "not similar" << endl;
}

void Experiment4() {
    string cand;
    cin >> cand;
    size_t i = cand.find_first_of("!?[&");
    cout << i << endl;
}

int main(int argc, const char * argv[]) {
    //Experiment1();
    //Experiment2();
    //Experiment3();
    Experiment4();
    return 0;
}
