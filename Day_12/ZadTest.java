import java.io.File;
import java.io.FileNotFoundException;
import java.util.Arrays;
import java.util.LinkedList;
import java.util.Queue;
import java.util.Scanner;
import java.util.Stack;

public class ZadTest {
    public static int traverseGrid(char[][] grid, boolean[][] taken, int x, int y) {
        int steps = 0;
        Queue<Integer[]> nodes = new LinkedList<>();
        nodes.add(new Integer[] { x, y });
        int level = 1;
        int t = 0;
        int counter = 0;
        taken[x][y] = true;
        boolean isFound = false;
        while (nodes.size() != 0) {
            Integer[] curr = nodes.poll();
            t++;

            int cx = curr[0], cy = curr[1];
            // System.out.println(cx + " " + cy);
            if (grid[cx][cy] == 'z' + 1) {
                isFound = true;
                break;
            }

            if (cx > 0 && !taken[cx - 1][cy]) {
                if (grid[cx][cy] == 'S'
                        || grid[cx][cy] - grid[cx - 1][cy] >= -1) {
                    taken[cx - 1][cy] = true;
                    System.out.printf("%c -> %c%n", grid[cx][cy], grid[cx - 1][cy]);

                    nodes.add(new Integer[] { cx - 1, cy });
                }
            }
            if (cx < 4 && !taken[cx + 1][cy]) {
                if (grid[cx][cy] == 'S'
                        || grid[cx][cy] - grid[cx + 1][cy] >= -1) {
                    taken[cx + 1][cy] = true;
                    System.out.printf("%c -> %c%n", grid[cx][cy], grid[cx + 1][cy]);
                    nodes.add(new Integer[] { cx + 1, cy });
                }
            }
            if (cy > 0 && !taken[cx][cy - 1]) {
                if (grid[cx][cy] == 'S'
                        || grid[cx][cy] - grid[cx][cy - 1] >= -1) {
                    taken[cx][cy - 1] = true;
                    System.out.printf("%c -> %c%n", grid[cx][cy], grid[cx][cy - 1]);
                    nodes.add(new Integer[] { cx, cy - 1 });
                }
            }
            if (cy < 7 && !taken[cx][cy + 1]) {
                if (grid[cx][cy] == 'S'
                        || grid[cx][cy] - grid[cx][cy + 1] >= -1) {
                    taken[cx][cy + 1] = true;
                    System.out.printf("%c -> %c%n", grid[cx][cy], grid[cx][cy + 1]);
                    nodes.add(new Integer[] { cx, cy + 1 });
                }
            }
            if (t == level) {
                steps++;
                level = nodes.size();
                t = 0;
            }
        }

        if (isFound)
            return steps;

        return -1;

    }

    public static void main(String[] args) {
        File input = new File("test-input.txt");
        try {
            Scanner s = new Scanner(input);
            char[][] grid = new char[5][8];
            int row = 0;
            int startX = 0, startY = 0;
            while (s.hasNextLine()) {
                String line = s.nextLine();
                for (int i = 0; i < line.length(); i++) {
                    grid[row][i] = line.charAt(i);
                    if (grid[row][i] == 'S') {
                        startX = row;
                        startY = i;
                    }
                    if (grid[row][i] == 'E') {
                        grid[row][i] = 'z' + 1;
                    }
                }
                row++;
            }
            s.close();
            boolean[][] taken = new boolean[5][8];
            System.out.println(traverseGrid(grid, taken, startX, startY));

        } catch (FileNotFoundException e) {
            e.printStackTrace();
        }
    }
}