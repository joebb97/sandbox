#include <iostream>
#include <stdio.h>
#include <unistd.h>

using namespace std;

void with_cout() {
  ios_base::sync_with_stdio(false);
  cout << "Portion1\n";
  sleep(1);
  cout << "Portion2";
  sleep(1);
  cout << "Portion3\n";
  cout << "Portion4\n";
  cout << "Portion5";
  sleep(1);
}

void with_printf() {
  printf("Portion1");
  sleep(1);
  printf("Portion2");
  sleep(1);
  printf("Portion3\n");
  printf("Portion4\n");
  printf("Portion5");
  sleep(1);
}

void getline_example() {
  string line;
  while (getline(cin, line)) {
      if (line == "") {
          cout << "We got him";
          break;
      }
    cout << line << "\n";
  }
}
int main() {
  with_printf();
  /* with_cout(); */
  /* getline_example(); */
  return 0;
}
