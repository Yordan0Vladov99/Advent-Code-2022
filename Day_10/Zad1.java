import java.io.File;
import java.io.FileNotFoundException;
import java.util.Arrays;
import java.util.HashSet;
import java.util.Scanner;

public class Zad1 {
    public static void main(String[] args) {
        File file = new File("input.txt");
        try {
            Scanner s = new Scanner(file);
            int counter = 0;
            int sum = 0;
            int val = 1;
            HashSet<Integer> strengthCycles = new HashSet<>(Arrays.asList(20, 60, 100, 140, 180, 220));
            while (s.hasNextLine()) {

                String line = s.nextLine();
                String[] commands = line.split(" ");
                counter++;
                // System.out.printf("%d : %d%n", counter, val);
                if (strengthCycles.contains(counter)) {
                    sum += val * counter;
                }
                if (commands[0].equals("addx")) {
                    counter++;
                    if (strengthCycles.contains(counter)) {
                        sum += val * counter;
                    }

                    val += Integer.parseInt(commands[1]);
                }

            }
            System.out.println(sum);
        } catch (FileNotFoundException e) {
            // TODO Auto-generated catch block
            e.printStackTrace();
        }

    }
}