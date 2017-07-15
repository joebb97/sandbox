package com.joey.sorts;
import java.util.*;
public class Sorts {
	
	public static void main(String [] args){
		int n = 11;
		ArrayList<Integer> arrayRandom = new ArrayList<Integer>(n);
		Random rand = new Random();
		rand.setSeed(System.currentTimeMillis());
		for (int i=0; i<n; i++)
		{
		    Integer r = rand.nextInt() % 256;
		    arrayRandom.add(r);
		}
		ArrayList<Sorter> sorts = new ArrayList<Sorter>(Arrays.asList(/*new SystemSorter(), new BubbleSorter(), 
														new SelectionSorter(), new InsertionSorter(), */ new BogoSorter()));
		Collections.shuffle(sorts);
		for (Sorter sorter: sorts){
			test_sort(sorter, new ArrayList<Integer>(arrayRandom));
			//test_sort(sorter, arrayRandom);
		}
	}
	
	private static <T extends Comparable<T>> void test_sort(Sorter sorter, List<T> original){
		long start = System.currentTimeMillis();
		sorter.sort(original);
		long end = System.currentTimeMillis();
		if (check_sorted(original)){
			System.out.println(sorter + " took " + (end - start) * Math.pow(10, -3) + " seconds");
		}
		else {
			System.out.println(sorter + " failed!");
		}
	}
	
	private static <T extends Comparable<T>> boolean check_sorted(List<T> list){
		for (int i = 0; i < list.size() - 1; ++i){
			if (list.get(i).compareTo(list.get(i + 1)) > 0){
				return false;
			}
		}
		return true;
	}
	
	private static interface Sorter {
		public <T extends Comparable<T>> void sort(List<T> unsorted);
	}
	
	private static class BogoSorter implements Sorter{
		public String toString(){
			return "Bogo Sorter";
		}
		
		public <T extends Comparable<T>> void sort(List<T> unsorted){
			while (!check_sorted(unsorted)){
				Collections.shuffle(unsorted);
			}
		}
		
	}
	
	private static class MergeSorter implements Sorter{
		public String toString(){
			return "Merge Sorter";
		}
		
		public <T extends Comparable<T>> void sort(List<T> unsorted){
			
		}
		
	}
	
	private static class SystemSorter implements Sorter {
		public String toString(){
			return "System Sorter";
		}
		public <T extends Comparable<T>> void sort(List<T> unsorted){
			Collections.sort(unsorted);
		}
	}
	
	private static class BubbleSorter implements Sorter { 
		public String toString(){
			return "Bubble Sorter";
		}
		
		public <T extends Comparable<T> > void sort(List<T> unsorted){
			boolean swapped = false;
			for (int i = 0; i < unsorted.size(); ++i){
				for (int j = unsorted.size() - 1; j > i; --j){
					if (unsorted.get(j).compareTo(unsorted.get(j - 1)) < 0){
						swapped = true;
						Collections.swap(unsorted, j, j - 1);
					}
				}
				if (!swapped) break;
				swapped = false;
			}
		}
	}
	
	private static class InsertionSorter implements Sorter {
		public String toString(){
			return "Insertion Sorter";
		}
		
		public <T extends Comparable<T>> void sort(List<T> unsorted){
			for (int i = 1; i < unsorted.size(); ++i){
				for (int j = i; j > 0; --j){
					if (unsorted.get(j).compareTo(unsorted.get(j - 1))< 0){
						Collections.swap(unsorted, j, j-1);
					}
					else {
						break;
					}
				}
			}
		}
	}
	
	private static class SelectionSorter implements Sorter {
		public String toString(){
			return "Selection Sorter";
		}
		public <T extends Comparable<T>> void sort(List<T> unsorted){
			for (int i = 0; i < unsorted.size(); ++i){
				int smallest_pos = i;
				for (int j = i; j < unsorted.size(); ++j){
					if (unsorted.get(smallest_pos).compareTo(unsorted.get(j)) > 0){
						smallest_pos = j;
					}
				}
				Collections.swap(unsorted, i, smallest_pos);
			}
		}
	}
}
