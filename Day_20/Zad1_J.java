package Day_20;

import java.io.File;
import java.io.FileNotFoundException;
import java.util.ArrayList;
import java.util.Scanner;

public class Zad1_J {
    public static void main(String[] args) {
        File file = new File("./Day_20/input.txt");
        try {
            Scanner s = new Scanner(file);
            ArrayList<Integer> nums = new ArrayList<>();
            while (s.hasNextLine()) {
                nums.add(Integer.parseInt(s.nextLine()));
            }

            ArrayList<Integer> decrypted = new ArrayList<>(nums);

            for (int i = 0; i < nums.size(); i++) {
                // System.out.println(decrypted);
                int steps = nums.get(i);
                int index = decrypted.indexOf(steps);

                int inc = steps > 0 ? 1 : -1;

                for (int j = 0; j < Math.abs(steps); j++) {
                    int temp = decrypted.get(index);
                    int next = Math.floorMod(index + inc, decrypted.size());

                    decrypted.set(index, decrypted.get(next));
                    decrypted.set(next, temp);
                    index = next;
                }

            }
            // System.out.println(decrypted);

            int sum = 0;
            int zIndex = decrypted.indexOf(0);
            for (int j = 1; j < 4; j++) {
                sum += decrypted.get(Math.floorMod((zIndex + j * 1000), decrypted.size()));
            }
            System.out.println(sum);

        } catch (FileNotFoundException e) {
            // TODO Auto-generated catch block
            e.printStackTrace();
        }

    }
}