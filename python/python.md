# A collection of useful Python tricks

Some cool tricks to remember

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