import java.io.File;
import java.io.FileNotFoundException;
import java.util.LinkedList;
import java.util.Queue;
import java.util.Scanner;
import java.util.Stack;

public class Zad {
    public static int traverseGrid(char[][] grid, boolean[][] taken, int x, int y, int dx, int dy) {
        int steps = 0;
        Queue<Integer[]> nodes = new LinkedList<>();
        nodes.add(new Integer[] { x, y });
        int level = 1;
        boolean isFound = false;
        taken[x][y] = true;
        int t = 0;
        while (nodes.size() != 0) {
            Integer[] curr = nodes.poll();
            t++;

            int cx = curr[0], cy = curr[1];
            if (cx == dx && cy == dy) {
                isFound = true;
                break;
            }

            if (cx > 0 && !taken[cx - 1][cy]) {
                if (grid[cx][cy] - grid[cx - 1][cy] >= -1) {
                    taken[cx - 1][cy] = true;
                    nodes.add(new Integer[] { cx - 1, cy });
                }
            }
            if (cx < 40 && !taken[cx + 1][cy]) {
                if (grid[cx][cy] - grid[cx + 1][cy] >= -1) {
                    taken[cx + 1][cy] = true;
                    nodes.add(new Integer[] { cx + 1, cy });
                }
            }
            if (cy > 0 && !taken[cx][cy - 1]) {
                if (grid[cx][cy] - grid[cx][cy - 1] >= -1) {
                    taken[cx][cy - 1] = true;

                    nodes.add(new Integer[] { cx, cy - 1 });
                }
            }
            if (cy < 65 && !taken[cx][cy + 1]) {
                if (grid[cx][cy] - grid[cx][cy + 1] >= -1) {
                    taken[cx][cy + 1] = true;

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
        File input = new File("input.txt");
        try {
            Scanner s = new Scanner(input);
            char[][] grid = new char[41][66];
            int row = 0;
            int startX = 0, startY = 0;
            int endX = 0, endY = 0;

            while (s.hasNextLine()) {
                String line = s.nextLine();
                for (int i = 0; i < line.length(); i++) {
                    grid[row][i] = line.charAt(i);
                    if (grid[row][i] == 'S') {
                        grid[row][i] = 'a';
                        startX = row;
                        startY = i;
                    }
                    if (grid[row][i] == 'E') {

                        grid[row][i] = 'z';
                        endX = row;
                        endY = i;
                    }
                }
                row++;
            }
            s.close();
            boolean[][] taken = new boolean[41][66];
            System.out.println(traverseGrid(grid, taken, startX, startY, endX, endY));

        } catch (FileNotFoundException e) {
            e.printStackTrace();
        }
    }
}