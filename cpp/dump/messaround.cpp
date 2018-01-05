#include <iostream>
using namespace std;

struct Catstruct {
    string name;
    string type;
    int age;
};

class Cat {
	public:
		string name, type;
		int age;
};

class Dog {
    string name;
    int age;
public:
    Dog(string name_)
    : name(name_), age(0) {}
    void setName(string inName)
    {
    	name = inName;
    }
    string getName() 
    {
    	return name;
    }
    void have_birthday()
    {
    	age++;
    }
    int getAge();
};

void printCats(Cat arr[], int size)
{
	for (int i = 0; i < size; ++i)
	{
		cout << arr[i].name << endl;
	}
}

int Dog::getAge()
{
	return age;
}

int getAge(){
	return 6;
}



void changeName(Cat * input)
{
	input->name = "suh";
}

void printStuff(ostream & os)
{
	os << "hello" << endl;
}

void readIn(istream & asdf)
{
	
}

int datSize(int * arr)
{
	return sizeof(arr);
}


class Emote{
public:
	int x = 3;
	Emote() : name("dufus") {
		cout << "Emote def ctor" << endl;
	}

	Emote(string in) : name(in) {
		cout << "Emote string ctor" << endl;
	}

	string getName(){
		return this->name;
	}

private:
	string name;
};

class Kappa : public Emote{
public:
	
	string getName(){
		return "asdf";
	}

};

class SubKappa : public Kappa{
public:
	
};


int main(int argc, char** argv)
{

	// printStuff(cout);

	// int arr [] = {2, 2, 2, 2};
	// cout << datSize(arr);

	// Catstruct c;
	// Catstruct *c_ptr = &c;
	// //c.name = "Fred";
	// cout << c.name << endl;
	// cout << c_ptr->name << endl;

	// Catstruct a = {"john", "persian", 3};
	// Catstruct b = {"merlin", "kappa"};
	// cout << a.name << b.name << endl;
	// Dog kappa("steven");
	// cout << kappa.getName() << endl;
	// cout << kappa.getAge() << endl;
	// kappa.have_birthday();
	// cout << kappa.getAge() << getAge() << endl;

	//Emote forehead("4Head");
	// SubKappa asdf;
	// Kappa * kappa = &asdf;
	// cout << kappa->x << endl;

	//Emote & emref = forehead;
	//cout << emref.getName();
	//cout << kappa.getName() << kappa.x;

	// cout << forehead.getName() << endl << kappa.getName() << endl;

	// int a = 5;
	// int b = 10;
	// int * y = &a; 
	// int & z = b; 
	// y = z;
	// z = a;
	// y = &b; 
	// z += 1; 
	// *y = 0;

	// cout << a << endl;
	// cout << b << endl;
	// cout << *y << endl;
	// cout << z << endl;
    int i = 40;
    cout << -i << endl;
	return 0;

}
