// See https://aka.ms/new-console-template for more information

using System.Collections;

Console.WriteLine("Hello, World!");

int[] a = { 1, 2, 3 };
int[] b = { 1, 2, 3 };
Console.WriteLine($"{Packet.Comparray(a, b)}");

public struct Packet
{
    public List<Packet> Array;
    public enum CompareResult
    {
        OutOfOrder,
        InOrder
    }

    public static CompareResult Comparray(int[] shouldBeSmaller, int[] shouldBeBigger)
    {
        if (shouldBeBigger.Length != shouldBeSmaller.Length)
        {
            return shouldBeBigger.Length > shouldBeSmaller.Length
                ? CompareResult.InOrder
                : CompareResult.OutOfOrder;
        }

        return shouldBeSmaller.Where((t, i) => shouldBeBigger[i] < t).Any()
            ? CompareResult.OutOfOrder
            : CompareResult.InOrder;
    }

    public static object ParseLine(string line)
    {
        const char OPENING = '[';
        const char CLOSING = ']';
        var FoundOpened = 0;
        var CurrentLine = "";
        
        foreach (var c in line)
        {
            
        }
        
        
        return null;

    }
}