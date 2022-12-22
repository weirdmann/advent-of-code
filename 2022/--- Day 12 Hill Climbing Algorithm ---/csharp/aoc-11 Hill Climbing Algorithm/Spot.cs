using System.Diagnostics;
using System.Net;
using System.Net.Mail;
using System.Runtime.CompilerServices;
using System.Text;
using Microsoft.VisualBasic.CompilerServices;

namespace aoc_11_Hill_Climbing_Algorithm;

public struct Position
{
    public int X;
    public int Y;

    public Position(int x, int y)
    {
        X = x;
        Y = y;
    }

    public override bool Equals(object? obj)
    {
        if (obj is null or not Position) return false;
        return this == (Position)obj;
    }

    public override int GetHashCode()
    {
        return HashCode.Combine(X, Y);
    }

    public static bool operator ==(Position p1, Position p2)
    {
        return p1.X == p2.X && p1.Y == p2.Y;
    }

    public static bool operator !=(Position p1, Position p2)
    {
        return !(p1 == p2);
    }
}

public class AStarMap
// https://en.wikipedia.org/wiki/A*_search_algorithm
{
    public PriorityQueue<Spot, int> OpenSet = new();
    public Position StartPosition;
    public Position EndPosition;
    public Spot StartSpot;
    public Spot EndSpot;
    public Dictionary<Position, Spot> Spots = new();
    public int Columns, Rows;

    public void AddToOpenSet(Spot spot)
    {
        OpenSet.Enqueue(spot, spot.F);
    }

    public void Init(int cols, int rows, Position start, Position end)
    {
        StartPosition = start;
        EndPosition = end;
        Columns = cols;
        Rows = rows;

        for (var x = 0; x < cols; x++)
        {
            for (var y = 0; y < rows; y++)
            {
                var s = new Spot(x, y);
                s.H = Heuristic(s);
                Spots.Add(s.Position, s);
            }
        }

        

        OpenSet = new();
        StartSpot = Spots.GetValueOrDefault(StartPosition) ?? throw new InvalidOperationException();
        EndSpot = Spots.GetValueOrDefault(EndPosition) ?? throw new InvalidOperationException();
        StartSpot.G = 0;
        AddToOpenSet(StartSpot);
    }

    public void AddNeighbors()
    {
        Spot potentialNeighbor;
        for (var x = 0; x < Columns; x++)
        {
            for (var y = 0; y < Rows; y++)
            {
                var s = Spots.GetValueOrDefault(new Position() { X = x, Y = y }) ??
                        throw new InvalidOperationException();


                if (x < Columns - 1)
                {
                    potentialNeighbor = Spots.GetValueOrDefault(new Position() { X = x + 1, Y = y }) ??
                                        throw new InvalidOperationException();
                    if (potentialNeighbor.HeightNumeric <= s.HeightNumeric + 1 | potentialNeighbor.HeightChar == 'E')
                        s.Neighbors.Add(potentialNeighbor);
                }

                if (x > 0)
                {
                    potentialNeighbor = Spots.GetValueOrDefault(new Position() { X = x - 1, Y = y }) ??
                                        throw new InvalidOperationException();
                    if (potentialNeighbor.HeightNumeric <= s.HeightNumeric + 1 | potentialNeighbor.HeightChar == 'E')
                        s.Neighbors.Add(potentialNeighbor);
                }

                if (y < Rows - 1)
                {
                    potentialNeighbor = Spots.GetValueOrDefault(new Position() { X = x, Y = y + 1 }) ??
                                        throw new InvalidOperationException();
                    if (potentialNeighbor.HeightNumeric <= s.HeightNumeric + 1 | potentialNeighbor.HeightChar == 'E')
                        s.Neighbors.Add(potentialNeighbor);
                }

                if (y > 0)
                {
                    potentialNeighbor = Spots.GetValueOrDefault(new Position() { X = x, Y = y - 1 }) ??
                                        throw new InvalidOperationException();
                    if (potentialNeighbor.HeightNumeric <= s.HeightNumeric + 1 | potentialNeighbor.HeightChar == 'E')
                        s.Neighbors.Add(potentialNeighbor);
                }
            }
        }
    }

    public void FromString(String contents)
    {
        var split = contents.Split(Environment.NewLine);
        var rows = split.Length;
        var cols = split[0].Length;
        {
            Position? start = null;
            Position? end = null;

            var x = 0;
            var y = 0;
            foreach (var line in contents.Split(Environment.NewLine))
            {
                if (start is null)
                {
                    x = line.IndexOf('S');
                    if (x >= 0) start = new Position() { X = x, Y = y };
                }

                if (end is null)
                {
                    x = line.IndexOf('E');
                    if (x >= 0) end = new Position() { X = x, Y = y };
                }

                y++;
            }

            if (start is null || end is null) throw new InvalidDataException();

            Init(cols, rows, (Position)start, (Position)end);
            
            
        }


        for (var y = 0; y < this.Rows; y++)
        {
            for (var x = 0; x < this.Columns; x++)
            {
                var current = this.Spots.GetValueOrDefault(new Position() { X = x, Y = y }) ??
                              throw new InvalidDataException();

                current.HeightChar = split[y][x];
            }
        }
        
        AddNeighbors();
    }

    public Queue<Spot>? ReconstructPath(Spot lastSpot)
    {
        var path = new Queue<Spot>();
        var current = lastSpot;

        foreach (KeyValuePair<Position, Spot> spot in Spots)
        {
            spot.Value.IsPath = false;
        }
        
        current.IsPath = true;

        while (current.Previous is not null)
        {
            path.Enqueue(current);
            current = current.Previous;
            current.IsPath = true;
        }

        return path;
    }

    public Result Main()
    {
        int score;
        if (OpenSet.Count == 0)
        {
            return new Result()
            {
                Finished = true,
                Success = false,
                Path = null
            };
        }

        var currentSpot = OpenSet.Dequeue() ?? throw new InvalidOperationException();


        if (currentSpot.Position == EndPosition)
        {
            // Success!
            return new Result()
            {
                Finished = true,
                Success = true,
                Path = ReconstructPath(currentSpot)
            };
        }

        foreach (var neighbor in currentSpot.Neighbors)
        {
            score = currentSpot.G + 1;
            if (score >= neighbor.G) continue;

            // this is a better path
            neighbor.Previous = currentSpot;
            neighbor.G = score;
            neighbor.F = score + Heuristic(neighbor);

            var exists = false;
            foreach (var (element, p) in OpenSet.UnorderedItems)
            {
                if (element.Position == neighbor.Position)
                {
                    exists = true;
                    break;
                }
            }

            if (!exists)
            {
                AddToOpenSet(neighbor);
            }
        }


        return new Result()
        {
            Finished = false,
            Success = false,
            Path = ReconstructPath(currentSpot)
        };
        
        
    }

    public int Heuristic(Spot s)
    {
        return (int)Math.Sqrt((Math.Pow(s.Position.X - EndPosition.X, 2) + Math.Pow(s.Position.Y - EndPosition.Y, 2)));
    }
    
    public String Visualize()
    {
        var sb = new StringBuilder();
        var line = new StringBuilder();

        for (var y = 0; y < Rows; y++)
        {
            for (var x = 0; x < Columns; x++)
            {
                var s = Spots.GetValueOrDefault(new Position() { X = x, Y = y }) ??
                        throw new InvalidOperationException($"x:{x}, y:{y}");
                var c = s.HeightChar.ToString();
                if (s.IsPath) c = " ";//c = s.HeightChar.ToString().ToUpper();
                line.Append(c);
                //line.Append(' ');
            }

            line.Append('\n');
            sb.Append(line.ToString());
            line.Clear();
        }

        return sb.ToString();
    }

    public struct Result
    {
        public bool Finished;
        public bool Success;
        public Queue<Spot>? Path;
    }
}

public class Spot
{
    public char HeightChar = 's';
    public uint HeightNumeric => HeightChar.ToString().ToLower()[0];

    public bool IsPath = false;

    public Position Position;

    public int F = int.MaxValue;

    public int G = int.MaxValue;

    public int H = int.MaxValue;

    private int _h;
    private bool _updateH;

    public List<Spot> Neighbors;
    public Spot? Previous;

    public Spot(int x, int y)
    {
        Position = new Position(x, y);
        _updateH = true;

        Neighbors = new List<Spot>();
    }
}