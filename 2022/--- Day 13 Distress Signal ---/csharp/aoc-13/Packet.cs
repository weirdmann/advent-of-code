using System.Text;

namespace aoc_13;

public struct Packet
{
    public int Index;
    public Element Left;
    public Element Right;

    public struct Element
    {
        public bool IsArray;
        public int Value;
        public List<Element>? Elements;

        public Element(int v)
        {
            Value = v;
            IsArray = false;
        }

        public Element(List<Element> el)
        {
            Elements = el;
            IsArray = true;
        }

        public static bool operator >(Element left, Element right)
        {
            var leftIsEmpty = left.Elements.Count == 0;
            var rightIsEmpty = right.Elements.Count == 0;

            if (leftIsEmpty && rightIsEmpty) return false;
            if (leftIsEmpty && !rightIsEmpty) return false;
            if (!leftIsEmpty && rightIsEmpty) return true;


            var isLeftNum = left.Elements.Count == 1;
            var isRightNum = right.Elements.Count == 1;

            return false;
        }

        public static bool operator <(Element left, Element right)
        {
            return !(left > right);
        }

        public static Element FromString(string source)
        {
            source = source.Replace("[", "");
            source = source.Replace("]", "");

            if (source.Length == 0) return new Element(new List<Element>());

            var split = source.Split(',');
            var values = split.Select(int.Parse);
            return new Element(values.Select(value => new Element(value)).ToList());
            return split.Length switch
            {
                1 => new Element(int.Parse(split[0])),
                _ => new Element(values.Select(value => new Element(value)).ToList())
            };
        }
        
        
        public static (string, Element, string) ParseFromString(string line)
            {
                const char opening = '[';
                const char closing = ']';
                const char comma = ',';
                var nestingLevel = 0;
                var lookingForClosing = false;
                var currentLine = line;
                var memory = new StringBuilder();
                var bracketStack = new Stack<int>();
                var commaStack = new Stack<int>();
                var index = 0;
                while (currentLine.Length > 0)
                {
                    var addNewLine = false;
                    var currentChar = currentLine[0];
                    currentLine = currentLine.Remove(0, 1);
                    switch (currentChar)
                    {
                        case opening:
                            if (!lookingForClosing)
                            {
                                lookingForClosing = true;
                                index = 0;
                                addNewLine = true;
                            }
                            else
                            {
                                nestingLevel += 1;
                            }
        
                            break;
                        case closing:
                            if (!lookingForClosing)
                            {
                                Console.WriteLine("unmatched parent");
                                break;
                            }
        
                            nestingLevel -= 1;
                            switch (nestingLevel)
                            {
                                case 0:
                                    //memory.Append(Environment.NewLine);
                                    break;
                                case -1:
                                    lookingForClosing = false;
                                    nestingLevel = 0;
                                    memory.Append(Environment.NewLine);
                                    break;
                            }
        
                            break;
                        case comma:
                            index += 1;
                            break;
                    }
                    
                    memory.Append(currentChar);
                    if (addNewLine) memory.Append(Environment.NewLine);
                }
        
                Console.WriteLine($"{line}:\n{memory}\n");
                return ("", FromString(memory.ToString()), "");
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

    public static CompareResult CompareArray(Element shouldBeSmaller, Element shouldBeBigger)
    {
        if (shouldBeBigger.Elements.Count != shouldBeSmaller.Elements.Count)
            return shouldBeBigger.Elements.Count > shouldBeSmaller.Elements.Count
                ? CompareResult.InOrder
                : CompareResult.OutOfOrder;

        return shouldBeSmaller.Elements.Where((t, i) => shouldBeBigger.Elements[i] < t).Any()
            ? CompareResult.OutOfOrder
            : CompareResult.InOrder;
    }


    
}