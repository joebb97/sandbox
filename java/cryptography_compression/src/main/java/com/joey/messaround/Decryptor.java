package com.joey.messaround;

import java.io.ByteArrayInputStream;
import java.io.ByteArrayOutputStream;
import java.io.IOException;
import java.security.InvalidAlgorithmParameterException;
import java.security.InvalidKeyException;
import java.security.Key;
import java.security.NoSuchAlgorithmException;
import java.security.spec.InvalidKeySpecException;
import java.util.zip.GZIPInputStream;

import javax.crypto.BadPaddingException;
import javax.crypto.Cipher;
import javax.crypto.CipherInputStream;
import javax.crypto.IllegalBlockSizeException;
import javax.crypto.NoSuchPaddingException;
import javax.crypto.SecretKeyFactory;
import javax.crypto.spec.IvParameterSpec;
import javax.crypto.spec.PBEKeySpec;
import javax.crypto.spec.SecretKeySpec;

public class Decryptor {
	
	private static final String decryptKey = "&5wQ\"gkd+\'&zA\'Zzoj9&lNr\'R884z1!q*\'E4{@a(/_8I:h}|y4"; 
	private static final String decryptSalt = "9GizM36xXnIR+6j8](oo3B!OSj7zFV)4cN6r2z465X6uJzL32";
	private static final byte [] iv = {(byte) 116, (byte) 42, (byte) 163, (byte) 99, (byte) 127, (byte) 114, (byte) 106, (byte) 46, (byte) 69, (byte) 13, (byte) 119, (byte) 216, (byte) 6, (byte) 76, (byte) 170, (byte) 10};
	
	public static byte [] decrypt(byte [] input) throws NoSuchAlgorithmException, InvalidKeySpecException, NoSuchPaddingException, InvalidKeyException, InvalidAlgorithmParameterException, IllegalBlockSizeException, BadPaddingException, IOException{
		PBEKeySpec keySpec = new PBEKeySpec(decryptKey.toCharArray(), decryptSalt.getBytes(), 1000, 128);
    	SecretKeyFactory factory = SecretKeyFactory.getInstance("PBKDF2WithHmacSHA1");
    	Key dkey = factory.generateSecret(keySpec);
    	IvParameterSpec initv = new IvParameterSpec(iv);
    	SecretKeySpec spec = new SecretKeySpec(dkey.getEncoded(), "AES");
    	Cipher cipher = Cipher.getInstance("AES/CBC/PKCS5Padding");
    	cipher.init(Cipher.DECRYPT_MODE, spec, initv);
    	ByteArrayInputStream bais = new ByteArrayInputStream(input);
    	bais.skip(16);
    	GZIPInputStream gzis = new GZIPInputStream( new CipherInputStream(bais, cipher));
    	ByteArrayOutputStream bos = new ByteArrayOutputStream();
    	//gzis.skip(11);
    	byte [] buffer = new byte[1028];
    	int length;
    	while ((length = gzis.read(buffer)) > 0){
    		bos.write(buffer, 0, length);
    	}
    	
    	gzis.close();
		return bos.toByteArray();
	}

}
