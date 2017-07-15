package com.joey.messaround;

import java.util.ArrayList;
//import java.util.HashMap;

public class Runner {

	
	public static void main(String [] args)
	{
		//HashMap <Integer, String> names;
		ArrayList<Byte> a = new ArrayList<Byte>();
		a.add(new Byte((byte) 5));
		PBV.Convert(a);
		byte [] f = {4, 88, 21};
		PBV.Convert2(f);
		System.out.println(f[0]);
		for (Byte b: f){
			System.out.println(b);
			if (b > 33);
			continue;
		}
		//System.out.println(a);
	}
}
