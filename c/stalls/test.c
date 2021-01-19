#include "util.h"
#include <assert.h>

void test1() {
  int arr[13] = {1, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1};
  int expected = 1;
  assert(calculate_social_distance(arr, 13) == expected);
}

void test2() {
  int arr[12] = {0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1};
  int expected = 1;
  assert(calculate_social_distance(arr, 12) == expected);
}

void test3() {
  int arr[12] = {1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0};
  int expected = 1;
  assert(calculate_social_distance(arr, 12) == expected);
}

void test4() {
  int arr[14] = {1, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 1, 0};
  int expected = 1;
  assert(calculate_social_distance(arr, 14) == expected);
}

void test5() {
  int arr[12] = {1, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1};
  int expected = 0;
  assert(calculate_social_distance(arr, 12) == expected);
}

int main() {
  test1();
  test2();
  test3();
  test4();
  test5();
  return 0;
}
