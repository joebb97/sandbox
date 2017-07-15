package com.joey.messaround;

import java.io.ByteArrayInputStream;
import java.io.ByteArrayOutputStream;
import java.io.IOException;
import java.io.UnsupportedEncodingException;
import java.security.InvalidAlgorithmParameterException;
import java.security.InvalidKeyException;
import java.security.NoSuchAlgorithmException;
import java.security.spec.InvalidKeySpecException;
import java.util.Arrays;
import java.util.zip.GZIPInputStream;

import javax.crypto.BadPaddingException;
import javax.crypto.IllegalBlockSizeException;
import javax.crypto.NoSuchPaddingException;

public class AESStuff {
	
	public static void main(String [] args) throws InvalidKeyException, NoSuchAlgorithmException, InvalidKeySpecException, NoSuchPaddingException, InvalidAlgorithmParameterException, IllegalBlockSizeException, BadPaddingException, IOException{
		
		String str = "I NEED A ONE DANCE, GOT A HENNESSY IN MY HANDS";
		byte [] iv = {(byte) 116, (byte) 42, (byte) 163, (byte) 99, (byte) 127, (byte) 114, (byte) 106, (byte) 46, (byte) 69, (byte) 13, (byte) 119, (byte) 216, (byte) 6, (byte) 76, (byte) 170, (byte) 10};
		byte [] strBytes = str.getBytes("UTF-8");
		System.out.println(str);
		byte [] zipped = Zipper.zipBytes(strBytes);
		byte [] encrypt = Encryptor.encrypt(zipped);
		byte [] b = new byte[]{ 44, 33, 36, 67, 21, 56, 77, 12, 99, 101, 22, 45, 32, 10, 2, 3};
		ByteArrayOutputStream bos = new ByteArrayOutputStream();
		bos.write(b);
		bos.write(encrypt);
		System.out.println(new String(encrypt, "UTF-8"));
		System.out.println(new String(bos.toByteArray(), "UTF-8"));
		byte [] decrypt = Decryptor.decrypt(bos.toByteArray());
		System.out.println(new String(decrypt, "UTF-8"));
		int i = 3;
		
	}

}
