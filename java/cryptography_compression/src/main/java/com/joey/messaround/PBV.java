package com.joey.messaround;

import java.util.ArrayList;

public class PBV {

	public static void Convert(ArrayList b)
	{
		b.set(0, new Byte((byte) 4));
	}
	
	public static void Convert2(byte [] b)
	{
		b[0] = 0;
	}
}
