using System.Diagnostics;
using System.Text;

public struct Packet
{
    public int Index;
    public Element Left;
    public Element Right;

    public struct Element
    {
        public List<Element> Array;

        public static bool operator >(Element left, Element right)
        {
            bool leftIsEmpty = left.Array.Count == 0;
            bool rightIsEmpty = right.Array.Count == 0;

            if (leftIsEmpty && rightIsEmpty) return false;
            if (leftIsEmpty && !rightIsEmpty) return false;
            if (!leftIsEmpty && rightIsEmpty) return true;

            
            bool isLeftNum = left.Array.Count == 1;
            bool isRightNum = right.Array.Count == 1;

            return false;
        }

        public static bool operator <(Element left, Element right)
        {
            return !(left > right);
        }
    }

    public Packet(int index, Element left, Element right)
    {
        Index = index;
        Left = left;
        Right = right;
    }

    public enum CompareResult
    {
        OutOfOrder,
        InOrder
    }

    public static CompareResult Comparray(Element shouldBeSmaller, Element shouldBeBigger)
    {
        if (shouldBeBigger.Array.Count != shouldBeSmaller.Array.Count)
        {
            return shouldBeBigger.Array.Count > shouldBeSmaller.Array.Count
                ? CompareResult.InOrder
                : CompareResult.OutOfOrder;
        }

        return shouldBeSmaller.Array.Where((t, i) => shouldBeBigger.Array[i] < t).Any()
            ? CompareResult.OutOfOrder
            : CompareResult.InOrder;
    }

    public static int Parse(string contents)
    {
        const char OPENING = '[';
        const char CLOSING = ']';
        var FoundOpened = 0;
        var CurrentLine = "";

        var split = contents.Split($"{Environment.NewLine}{Environment.NewLine}");

        foreach (var packetPair in split)
        {
        }

        return 0;
    }

    public static string ParseSinglePacket(string line)
    {
        const char OPENING = '[';
        const char CLOSING = ']';
        var foundOpened = 0;
        var lookingForClosing = false;
        var currentLine = line;
        var memory = new StringBuilder();
        while (currentLine.Length > 0)
        {
            var currentChar = currentLine[0];
            currentLine = currentLine.Remove(0, 1);
            memory.Append(currentChar);
            switch (currentChar)
            {
                case OPENING:
                    if (!lookingForClosing)
                    {
                        lookingForClosing = true;
                        memory.Append(Environment.NewLine);
                    }
                    else
                    {
                        foundOpened += 1;
                    }
                    break;
                case CLOSING:
                    if (!lookingForClosing)
                    {
                        Console.WriteLine("unmatchad parent");
                        break;
                    }
                    
                    foundOpened -= 1;
                    switch (foundOpened)
                    {
                        case 0:
                            memory.Append(Environment.NewLine);
                            break;
                        case -1:
                            lookingForClosing = false;
                            foundOpened = 0;
                            break;
                    }
                    break;
            }
        }
        Console.WriteLine(memory.ToString());
        return memory.ToString();
    }
}