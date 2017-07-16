#include <iostream>
#include <string>
#include <vector>
#include <cmath>
#include <map>
#include <cstdlib>
#include <sstream>
#include <queue>
#include <functional>

using namespace std;

size_t get_c_string_length(const char* input) {
  size_t ret = 0;
  while (*input != '\0'){
    ++ret;
    ++input;
  }
  return ret;
}
void print_vec(vector<size_t> vec){
  for (auto & i : vec){
    cout << i << " ";
  }
  cout << endl;
}
char* append_c_string(const char* first, const char* second) {
  //
  // TODO: fill this in
  //
  size_t first_length = get_c_string_length(first);
  size_t second_length = get_c_string_length(second);
  size_t capacity = first_length + second_length;
  char* result = new char[capacity];
  size_t result_index = 0;
  for (size_t i = 0; i < first_length; ++i, ++result_index){
    result[result_index] = first[i];
  }
  for (size_t i = 0; i < second_length; ++i, ++result_index){
    result[result_index] = second[i];
  }
  result[result_index] = '\0';
  //
  // TODO: fill this in
  //
  return result;
}

struct Comparator {
  bool operator() (int lhs, int rhs) const {
    int lhsParity = lhs % 2;
    int rhsParity = rhs % 2;
    if (lhsParity < rhsParity) {
      return true;
    }
    else if (lhsParity > rhsParity) {
      return false;
    }
    else {
      return lhs < rhs;
    }
  }
};

void test_comparator() {
  std::priority_queue<int, vector<int>, Comparator> pq;
  for (int n: {4, 10, 2, 20, 9, 15, 6, 3, 1, 17}){
    pq.push(n);
  }
  while (!pq.empty()){
    cout << pq.top() << " ";
    pq.pop();
  }
  cout << endl; 
}

void test_min_pq() {
  std::priority_queue<int, vector<int>, std::greater<int>> pq;
  pq.push(10);
  pq.push(2);
  pq.push(5);
  pq.push(11);
  pq.push(3);
  pq.pop();
  pq.pop();
  pq.push(-3);
  pq.push(5);
  pq.pop();
  pq.push(5);
  pq.push(-99);
   while (!pq.empty()){
    cout << pq.top() << " ";
    pq.pop();
  }
  cout << endl; 
}

void merge2(vector<size_t> &first, vector<size_t> &second) {
  size_t endFirst = first.size();
  size_t currentLoc = second.size() - 1;

  //size_t secret = 0;

  //Shift second down n elements, where n is the size of first
  for (; currentLoc >= first.size(); --currentLoc){
    second[currentLoc] = second[currentLoc - first.size()];
    second[currentLoc - first.size()] = 0;
  }
  ++currentLoc;

  cout << endl;
  print_vec(second);
  cout << endl;

  size_t startFirst = 0;
  for (size_t i = 0; i < second.size(); ++i){
    if (currentLoc == second.size()){
      second[i] = first[startFirst++];
    }
    else if (startFirst == endFirst){
      break;
    }
    else {
      second[i] = (first[startFirst] <= second[currentLoc] ? 
             first[startFirst++] : second[currentLoc++]);
    }
  }
}
void test_merge2(){
  vector<size_t> A = {4, 8, 15, 16, 23, 42};
  vector<size_t> B = {1, 5, 19, 50, 74, 0, 0, 0, 0, 0, 0};
  print_vec(A);
  print_vec(B);
  merge2(A, B);
  print_vec(B);
}

int main(){
  test_merge2();  
}