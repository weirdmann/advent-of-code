
using System.Text;

namespace aoc_7
{
    public struct Result<T>
    {
        public bool ok;
        public T value;

        public static Result<T> Success(T value)
        {
            return new Result<T> { ok = true, value = value };
        }

        public static Result<T> Fail(T value)
        {
            return new Result<T> { ok = false, value = value };
        }

        public Result<T> OnSuccess(Action<T> f)
        {
            if (!ok) return this;

            if (f != null) f.DynamicInvoke(value);
            return this;
        }

        public Result<T> OnFail(Action<T> f)
        {
            if (ok) return this;

            if (f != null) f.DynamicInvoke(value);
            return this;
        }
    }
    public class ElFileSystem
    {
        public class Explorer
        {
            public Directory Root;
            public Directory Cwd;

            public Explorer(Directory root)
            {
                Root = root;
                Cwd = Root;
            }

            public Result<Directory> Execute(string command)
            {
                var split = command.Split(' ');

                if (split[0].Equals('$'))
                {
                    if (split[1].Equals("cd"))
                    {

                    }
                }

                return Result<Directory>.Success(Cwd);
            }
        }

        public class File
        {
            public string Name { get; set; }
            public UInt64 Size { get; set; }

            public File(string name, UInt64 size)
            {
                Name = name;
                this.Size = size;
            }
        }

        public class Directory
        {
            public string Name { get; set; }
            public UInt64 Size
            {
                get
                {
                    UInt64 counted_size = 0;
                    foreach (var pair in Files)
                    {
                        counted_size += pair.Value.Size;
                    }
                    return counted_size;
                }
            }
            public UInt64 size_recursive
            {
                get
                {
                    UInt64 counted_size = Size;
                    foreach (var pair in Subdirectories)
                    {
                        counted_size += pair.Value.size_recursive;
                    }
                    return counted_size;
                }
            }

            public Dictionary<string, Directory> Subdirectories { get; set; }
            public Dictionary<string, File> Files { get; set; }
            public Directory? parent { get; set; }

            public Directory(string name, Directory? parent)
            {
                this.Name = name;
                this.parent = parent;
                this.Subdirectories = new();
                this.Files = new();
            }

            public Result<Directory?> mkdir(string name)
            {
                var new_directory = new Directory(name, this);
                if (!this.Subdirectories.TryAdd(name, new_directory))
                {
                    return Result<Directory?>.Fail(new_directory);
                }
                return Result<Directory?>.Success(new_directory);
            }

            public Result<File> AddFile(File file)
            {
                if (!this.Files.TryAdd(file.Name, file))
                {
                    return Result<File>.Fail(file);
                }
                return Result<File>.Success(file);
            }

            override public string ToString()
            {
                var tree = new StringBuilder();
                foreach (var pair in this.Subdirectories)
                {
                    tree.Append(string.Format("dir {0}\n", pair.Value.Name));
                }
                foreach (var pair in this.Files)
                {
                    tree.Append(string.Format("{0,10}\t{1,-0}\n", pair.Value.Size, pair.Value.Name));
                }
                return tree.ToString();
            }

            public string getTree()
            {
                var tree = new StringBuilder();
                foreach (var pair in this.Subdirectories)
                {
                    tree.Append(string.Format("\tdir {0}{1}\n", pair.Value.Name, pair.Value.getTree()));
                }
                foreach (var pair in this.Files)
                {
                    tree.Append(string.Format("{0,10}\t{1,-0}\n", pair.Value.Size, pair.Value.Name));
                }
                return tree.ToString();
            }
        }

        public struct ParsedCommand
        {
            public string str = "";
            public string[] lines;
            public string cd_to = "";
            public bool is_ls = false;
            public List<string> directories = new();
            public Dictionary<string, ulong> files = new();
            public ParsedCommand(string str)
            {
                this.str = str;
                lines = str.Split(new string[] { Environment.NewLine }, StringSplitOptions.None);
                var cd = lines[0].Split(' ');
                if (cd.Length != 3) throw new Exception($"bad command: {cd}");
                if (cd[0] != "&" & cd[1] != "cd") throw new ArgumentException($"not cd: {cd}");
                cd_to = cd[2];
                if (lines.Length < 2) return;
                is_ls = true;
                var ls = lines[1].Split(' ');
                if (ls.Length != 2) return;
                if (ls[0] != "&" & ls[1] != "ls") throw new ArgumentException($"not ls: {ls}");

                foreach (var line in lines[2..])
                {
                    var c = line.Split(' ');
                    if (c.Length != 2) break;
                    if (c[0] == "dir")
                    {
                        directories.Add(c[1]);
                    }
                    else
                    {
                        files.Add(c[1], ulong.Parse(c[0]));
                    }
                }
            }

            public static List<ParsedCommand> ParseFile(string contents)
            {
                var lines = contents.Split(new string[] { Environment.NewLine }, StringSplitOptions.None);
                var currentCommand = new StringBuilder();
                var commandAvailable = false;
                var parsedcommands = new List<ParsedCommand>();

                foreach (var line in lines)
                {
                    if (line.StartsWith("$ cd") | line.Equals(Environment.NewLine, StringComparison.InvariantCulture))
                    {
                        if (commandAvailable) parsedcommands.Add(new ParsedCommand(currentCommand.ToString()));
                        commandAvailable = true;
                        currentCommand.Clear();
                        currentCommand.AppendLine(line);
                        continue;
                    }
                    currentCommand.AppendLine(line);
                }
                if (commandAvailable) parsedcommands.Add(new ParsedCommand(currentCommand.ToString()));

                return parsedcommands;
            }
        }
    }
}
