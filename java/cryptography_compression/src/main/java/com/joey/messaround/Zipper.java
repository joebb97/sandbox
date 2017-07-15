package com.joey.messaround;

import java.io.ByteArrayOutputStream;
import java.io.IOException;
import java.util.zip.GZIPOutputStream;

public class Zipper {
	
	public static byte [] zipBytes(byte [] input) throws IOException{
		ByteArrayOutputStream bos = new ByteArrayOutputStream();
		GZIPOutputStream gzos = new GZIPOutputStream(bos);
		for (byte b: input){
			gzos.write(b);
		}
		gzos.close();
		bos.close();
		return bos.toByteArray();
		
	}

}
