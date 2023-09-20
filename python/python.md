# A collection of useful Python tricks

Some cool tricks to remember

## [1] Using ``argparse`` to parse arguments to a Python script
When you want your code to depend on some parameters or options that you want to see in the command line,
you can use the module ``argparse``.

```python
import argparse

parser = argparse.ArgumentParser()
parser.add_argument("--my_option", type=str, help="[str] The string option that you need to run the code")
args = parser.parse_args()

option = args.my_option     ## The flag you defined becomes a name for the args property that your code can read
```

An run your scripts as:
```
python my_script.py --my_option "like_this"
```

### [1.1] Parsing paths to files
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

### [1.2] Making some arguments *optional*
If you need to make some arguments *optional* and have some default values (for instance, if you want to ``--overwrite`` some file only if specified), you can define a ``default`` value. If you don't pass the flag, Python will assume the default value.

```python
import json
import argparse

parser = argparse.ArgumentParser()
parser.add_argument("--overwrite", type=bool, default=False, help="[bool] whether to overwrite the results")
args = parser.parse_args()

if args.overwrite:
    ## Overwrite the results [...]
    pass
```

## [2] Logging the Git commit ID of your code

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

## [3] Looping for data in a bunch of files
Let's say you have a load of JSON files in your folder and want to check whether some 'data' is within any of them

```python
import os
import json

def search_for_data(data_to_match):

    for root, _dirs, files in os.walk("./my_folder"):
        for this_file in files:     # Loop over the available files
            if this_file.startswith("results_"):        # Check if they start with the correct prefix
                file_path = os.path.join(root, this_file)
                with open(file_path, "r") as _file:     # Open the files and read the data
                    _data = json.load(_file)
                    for item in _data:      # Look for the data
                        if item["x"] == data_to_match:      # Data in this file matches what we are looking for
                            print(f"\nThis data '{data_to_match}' was found within '{file_path}'")
                            return file_path
    # Return None if you don't find anything
    return None
```

## [4] The ``datetime`` module

```python
from datetime import datetime

some_date = "2023-09-10"
my_day = datetime.strptime(some_date, "%Y-%m-%d")       # Convert some string to a datetime instance

one_day = datetime.timedelta(days=1)
previous_day = my_date - one_day
other_day = previous_day.strftime("%Y-%m-%d")           # Give a datetime instance a specific format
```

## [5] Adding ``logging``

more to come