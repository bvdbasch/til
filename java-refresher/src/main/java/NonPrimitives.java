import java.util.ArrayList;

public class NonPrimitives {
    public static void main(String[] args){
        String name = "Henk";
        System.out.println("Hello, I am " + name.toUpperCase());
        addExclamationPoint(name + " likes hot dogs");

        String exclaim = addExclamationPointAnotherWay("This local var will receive an exclamation point");
        System.out.println(exclaim + " " + name + " may have something to do with that...");

        Animal a = new Animal();
        String doggo = a.iAmDog();
        System.out.println(doggo);

        ArrayList<Integer> l = new ArrayList<Integer>();
        Boolean emptyList;
        if (l.isEmpty()) emptyList = true;
        else emptyList = false;

        System.out.println("The answer you are looking for is " + emptyList);
    }

    public static void addExclamationPoint(String s) {
        System.out.println(s + "!");
    }

    public static String addExclamationPointAnotherWay(String s) {
        return s + "!";
    }
}