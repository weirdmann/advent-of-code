using System;
using System.Collections;
using System.Collections.Generic;
using System.Data;
using System.Diagnostics;
using System.Linq;
using System.Runtime.CompilerServices;
using System.Text;
using System.Threading.Tasks;

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
                        counted_size += pair.Value.Size;
                    }
                    return counted_size;
                }
            }

            public Dictionary<String, Directory> Subdirectories { get; set; }
            public Dictionary<String, File> Files { get; set; }
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
                    tree.Append(String.Format("dir {0}\n", pair.Value.Name));
                }
                foreach (var pair in this.Files)
                {
                    tree.Append(String.Format("{0,10}\t{1,-0}\n", pair.Value.Size, pair.Value.Name));
                }
                return tree.ToString();
            }
        }

        public class PuzzleInput : IEnumerable<string>, IEnumerator<string>
        {
            public string Current => throw new NotImplementedException();

            object IEnumerator.Current => "test current";


            public void Dispose()
            {
            }

            public IEnumerable<string> GetEnumerator()
            {
                yield return "test";
            }

            public bool MoveNext()
            {
                throw new NotImplementedException();
            }

            public void Reset()
            {
                throw new NotImplementedException();
            }

            IEnumerator<string> IEnumerable<string>.GetEnumerator()
            {
                throw new NotImplementedException();
            }

            IEnumerator IEnumerable.GetEnumerator()
            {
                throw new NotImplementedException();
            }
        }


        public static System.Collections.IEnumerable SomeNumbers()
        {
            yield return 3;
            yield return 5;
            yield return 8;
        }
    }
}
