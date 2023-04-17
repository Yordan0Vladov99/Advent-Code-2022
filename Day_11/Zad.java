import java.io.File;
import java.io.FileNotFoundException;
import java.util.Arrays;
import java.util.HashMap;
import java.util.LinkedList;
import java.util.Queue;
import java.util.Scanner;

public class Zad {
    public static void main(String[] args) {
        File input = new File("input.txt");
        int[] monkeyInspections = { 0, 0, 0, 0, 0, 0, 0, 0 };
        HashMap<Integer, Queue<Long>> monkeys = new HashMap<>();
        int[] trueMonkeys = new int[8];
        int[] falseMonkeys = new int[8];
        String[] monkeyOperations = new String[8];
        int[] monkeyDivisors = new int[8];

        for (int monkey = 0; monkey < 8; monkey++) {
            monkeys.put(monkey, new LinkedList<>());
        }
        try {
            Scanner s = new Scanner(input);
            while (s.hasNextLine()) {
                String line = s.nextLine();
                if (!line.startsWith("Monkey"))
                    continue;

                int monkey = Integer.parseInt(line.split(" ")[1].split(":")[0]);
                line = s.nextLine();
                String[] startingItems = line.split(":")[1].split(",");
                for (String item : startingItems) {
                    monkeys.get(monkey).add(Long.parseLong(item.strip()));
                }

                line = s.nextLine();
                String operation = line.split("= ")[1];
                monkeyOperations[monkey] = operation;

                line = s.nextLine();
                int divisor = Integer.parseInt(line.split(": ")[1].split(" ")[2]);
                monkeyDivisors[monkey] = divisor;

                line = s.nextLine();
                int trueMonkey = Integer.parseInt(line.split(": ")[1].split(" ")[3]);
                trueMonkeys[monkey] = trueMonkey;

                line = s.nextLine();
                int falseMonkey = Integer.parseInt(line.split(": ")[1].split(" ")[3]);
                falseMonkeys[monkey] = falseMonkey;

            }
            int divAll = 1;
            for (int i : monkeyDivisors) {
                divAll *= i;
            }

            for (int i = 0; i < 10000; i++) {
                for (int monkey = 0; monkey < 8; monkey++) {
                    int trueMonkey = trueMonkeys[monkey];
                    int falseMonkey = falseMonkeys[monkey];
                    String[] operation = monkeyOperations[monkey].split(" ");
                    long divisor = monkeyDivisors[monkey];
                    monkeyInspections[monkey] += monkeys.get(monkey).size();
                    while (monkeys.get(monkey).size() != 0) {
                        long item = monkeys.get(monkey).poll();
                        long worry = 0;
                        long left = operation[0].equals("old") ? item : Long.parseLong(operation[0]);
                        long right = operation[2].equals("old") ? item : Long.parseLong(operation[2]);

                        if (operation[1].equals("+")) {
                            worry = left + right;
                        } else {
                            worry = left * right;
                        }
                        worry %= divAll;
                        if (Math.abs(worry % divisor) == 0) {
                            monkeys.get(trueMonkey).add(worry);
                        } else {
                            monkeys.get(falseMonkey).add(worry);
                        }

                    }
                }
                /*
                 * System.out.println();
                 * for (int j = 0; j <= 3; j++) {
                 * System.out.println("Monkey" + j + " : " + monkeys.get(j));
                 * }
                 */

            }

            Arrays.sort(monkeyInspections);
            System.out.println(Arrays.toString(monkeyInspections));
            long a = monkeyInspections[6];
            long b = monkeyInspections[7];
            System.out.println(a * b);
        } catch (FileNotFoundException e) {
            // TODO Auto-generated catch block
            e.printStackTrace();
        }

    }
}