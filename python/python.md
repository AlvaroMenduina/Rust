# A collection of useful Python tricks

Some cool tricks to remember

## Using ``argparse`` to parse arguments to a Python script
When you want your code to depend on some parameters or options that you want to see in the command line,
you can use the module ``argparse``.

```python
import argparse

parser = argparse.ArgumentParser()
parser.add_argument("--my_option", type=str, help="[str] The string option that you need to run the code")
args = parser.parse_args()

option = args.my_option     ## The flag you defined becomes a name for the args property that your code can read
```

### Parsing paths to files
If you need to read some files from within your script

```python
import json
import argparse

parser = argparse.ArgumentParser()
parser.add_argument("--my_file", type=argparse.FileType('r'), help="[str] The full path to the file to read")
args = parser.parse_args()

try:
    with open(args.my_file) as f:
        data = json.load(f)
except FileNotFoundError:
    print("There seems to be an error with the file")
```

## Logging the Git commit ID of your code

When you have a code that will evolve in the future, it may be important to understand what version of the code you used
when you produced old results. 

Using the Git-Python library [here](https://gitpython.readthedocs.io/en/stable/intro.html)

```python
import git

repo = git.Repo(search_parent_directories=True)
sha = repo.head.commit.hexsha
```

You can call ``git.Repo(search_parent_directories=True)`` to find a valid instance of a git repo. By adding that flag, we are telling it to search parent directories until it finds a valid one, so that we can use this code snippet within any python script in our package, rather than at the base of the repo.

The output ``repo`` object has this property called HEAD, which is a reference pointer to the latest commit. Then, the property ``commit`` represents that commit. The ``hexsha`` is just the string representation of the **git commit hash**.

We can use that hash to identify what commit we used when running our script, for troubleshooting or archival purposes.

```python
## ... get the commit hash
sha = repo.head.commit.hexsha

if __name__ == __main__():
    
    ## Run some analysis
    x = do_stuff()

    results = {'results': x, 'commit': sha}
    ## Save the results ...

```