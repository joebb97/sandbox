#include <iostream>
#include <string>
#include <vector>
#include <cmath>
#include <map>
#include <cstdlib>
#include <sstream>

using namespace std;

int suh();
int foo();
void printVec();
void printHex(string in){
	//cout << stoi(in, 16) << endl;		
}

void ptrRef(int * & a){
	*a = 45;
}

void swap(int * a, int * b){
	int temp = *a;
	*a = *b;
	*b  = temp;
}

void zeros_first(int arr[], int n ){
	int * zerosIndex = arr;
	int * start = arr;
	while (start < arr + n){

		if (*start == 0){
			swap(start, zerosIndex);
			++zerosIndex;
		}
		++start;
	}

}

//REQUIRES: "s" is a NULL-terminated C-string //EFFECTS: Returns a pointer to the first
// occurrence of "search" in "s".
// Returns NULL if not found.
char * strchr (char *s, char search){
	char * ret = nullptr;
	while (*s != '\0'){
		if (*s == search){
			ret = s;
			return ret;
		}
		s++;
	}
	return ret;
}

int foo(int a, int& b){
    a = b;
	a++;
	return b; 
}

int bar(int& b, int& a){
    return b = a;
}

void findIndex(int a[], int* ptr)
{
	int diff = ptr - a;
	cout << diff << endl;
}

void printHello(){
	cout << "Hellaw" << endl;
}

struct Circle
{
	double radius;
};

class Silly
{
	public:
		Silly() : myInt(0) {}

		int & getInt(){
			return myInt;
		}

		virtual void doSomething() = 0;

	private:
		int myInt;


};

class SubSilly : public Silly {
public:
		virtual void doSomething() {
			cout << "something";
		}
};

void doSomething(int * a)
{
	int b = *a;
	b++;
}

void foo(char * in){
	for (char *ptr = in; *ptr != '\0'; ptr++){
		*ptr = 's';
	}
}

void printVec(vector<string> mom)
{
	for (auto i: mom)
		cout << i;
	cout << endl;
}

void printVec(vector<int> mom)
{
	for (auto i: mom)
		cout << i;
	cout << endl;
}

const string & suhDude(string & in){
	in = "WTF";
	return in;
}

int main(int argc, char * argv[])
{
	
	// int r = x;
	// int *y = &x;
	// int &z = x;
	//r = 2;
	//cout << x << endl;
	//cout << r << endl;
	// cout << &x << endl;
	// cout << &z << endl;
	// cout << y << endl;
	
	// int i = 6;
	// cout << -30 - -45 << endl;
	// int meh[4] = {1, 2, 3, 4};
	// int* ptr = &meh[0];
	// findIndex(meh, ptr);
	// int xd = 3;
	// for (int i = 4; i < xd; i++)
	// 	cout << "KEK";
	// string asdf = "45 " + 4;
	// cout << asdf;

	// std::vector<int> v = {3, 5, 5, 8};
	// printVec(v);
	// v.erase(v.end() - 1);
	// printVec(v);
	// cout << v[v.size() - 1] << endl;
	
	SubSilly a;
	Silly * b = &a;
	b->doSomething();
	// cout << a.getInt() << endl;
	// int & b = a.getInt();
	// cout << b << endl;
	// //int c = a.getInt();
	// b++;
	// cout << b << endl;
	//cout << a.getInt() << endl;

	// string a = "Usage: euchre PACK_FILENAME [shuffle|noshuffle] POINTS_TO_WIN NAME1 TYPE1 NAME2 TYPE2 NAME3 TYPE3 NAME4 TYPE4\n";
	// ostringstream b;
 //        b << "Usage: euchre PACK_FILENAME [shuffle|noshuffle] POINTS_TO_WIN NAME1 " <<
 //          "TYPE1 NAME2 TYPE2 NAME3 TYPE3 NAME4 TYPE4" << endl;
 //    cout << (a == b.str()) << endl;
 //    cout << a << endl;
 //    cout << b.str() << endl;

// int a = 42;
// int b = 24;
//    cout << foo(a, b) << endl;
//    cout << a << "," << b << endl;
//    cout << bar(a, b) << endl;
//    cout << a << "," << b << endl;

	// int x = 33;
	// int * xptr = &x;
	// int * & ptrref = xptr;
	// ptrRef(ptrref);

	// double pi = 3.1415;
 //    double *p1 = &pi, p2 = pi;
 //    cout << p1 << endl;
 //    cout << *p2;
	
	// const char str[] = "eecs280 is awesome!";
	//  cout << str + 1 << endl;
	// char * ptr = strchr(str, 'a');
	// cout << "Address of str: "  << &str << endl;
	// cout << "Address of ptr: " << &ptr << endl;
	// cout << "Value of ptr: " << *ptr << endl;
	// string input = "";
	// cout << "Input your hex number ";
	// cin >> input;
	// cout << "Dec value: " << stoi(input, nullptr ,16) << endl;

	//int arr[10] = {3, 6, 0, 2, 0, 5, 8, 0, 0, 1};
	// int * start = arr;
	// int * end = arr + 10 - 1;
	// for (auto i: arr){
	// 	cout << i;
	// }
	// cout << endl;
	// bool meh = 0 == 0;
	// cout << meh << endl;
	//zeros_first(arr, 10);
	// cout << "{";
	// for (auto i: arr)
	// 	cout << i;
	// cout << "}";

	 // int x = 3;
	 // int m = 2;
	 // int * y = &x;
	 // int * z = &m;
	 // cout << (y  + 1) - z << endl;



	// char cstr[] = "SHUTTHEFRONTDOOR";
	// foo(cstr);
	// cout << cstr;
	// int x = '\0';
	// cout << x;
	// int x= 4;
	// int y = 3;
	// int z = -22 | 7;
	// cout << z << endl;
	// bool xd = (x & y);

	// cout << xd;
	// const string mimi = "mimi";
	// string & ref = mimi;
	
	// cout << ref << endl;

	// double x = .0;
	//double y = .2;
	//double z = x - y;
	// cout.precision(4);
	// //int x = 3;
	// double y = 50.12222222;
	// string x = "50.2345";
	// double z = stod(x);
	// cout << stod(x) << endl;
	// cout << y << endl;
	// cout << z << endl;

	// int x = 2;
	// int * z = &x;
	// int & y = *x;
	// y++;
	// cout << x << endl;
	// cout << y << endl;
	// cout << z << endl;

	// map<string, int> words;
	// words["xenon"] = 123;
	// words["abc"] = 2;
	// for (auto i: words){
	// 	cout << i.first << " " << i.second << endl;
	// }

	// int x = 3;
	// try {
	// 	x = 2;
	// 	cout << x << endl;
	// }
	// catch (...){
	// 	cout << "caught an exception somehow" << endl;
	// }
	// cout << x << endl;

	return 0;



}



int suh()
{
	//printHello();
	return 1;
}

int foo()
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
			//cout << twod[i][ja] << "\t";
		}
	}
	//cout << "\n";

	double first = 0.1;
	double second = 0.2;
	double result = first + second;

	double eps = 0.000001;

	cout << (abs(result - .3) < eps) << endl;
	return 0;

	//printVec(victor);
}



