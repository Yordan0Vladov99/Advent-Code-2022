import java.io.File;
import java.io.FileNotFoundException;
import java.util.Arrays;
import java.util.HashSet;
import java.util.Scanner;

public class Zad2 {
    public static void drawPixel(int counter, int[] sprite) {
        int pos = counter % 40;
        if (pos == 0) {
            System.out.println();
        }

        if (pos == sprite[0] || pos == sprite[1] || pos == sprite[2]) {
            System.out.print("#");
        } else {
            System.out.print(".");
        }
    }

    public static void main(String[] args) {
        File file = new File("input.txt");
        try {
            char[][] screen = new char[6][40];
            for (int i = 0; i < screen.length; i++) {
                Arrays.fill(screen[i], '.');
            }
            Scanner s = new Scanner(file);
            int counter = 0;
            int val = 1;
            int[] sprite = { 0, 1, 2 };

            while (s.hasNextLine()) {

                String line = s.nextLine();
                String[] commands = line.split(" ");

                drawPixel(counter, sprite);
                counter++;

                if (commands[0].equals("addx")) {
                    drawPixel(counter, sprite);
                    counter++;
                    val += Integer.parseInt(commands[1]);
                    sprite[0] = val - 1;
                    sprite[1] = val;
                    sprite[2] = val + 1;
                }

            }

        } catch (FileNotFoundException e) {
            // TODO Auto-generated catch block
            e.printStackTrace();
        }

    }
}