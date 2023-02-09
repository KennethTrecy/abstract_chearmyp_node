[![Library Tests](https://img.shields.io/github/actions/workflow/status/KennethTrecy/abstract_chearmyp_node/library.yml?style=for-the-badge)](https://github.com/KennethTrecy/abstract_chearmyp_node/actions/workflows/library.yml)
![GitHub lines](https://img.shields.io/github/license/KennethTrecy/abstract_chearmyp_node?style=for-the-badge)
![GitHub release (latest SemVer)](https://img.shields.io/github/v/release/KennethTrecy/abstract_chearmyp_node?style=for-the-badge&display_name=tag&sort=semver)
![GitHub closed issues count](https://img.shields.io/github/issues-closed/KennethTrecy/abstract_chearmyp_node?style=for-the-badge)
![GitHub pull request count](https://img.shields.io/github/issues-pr-closed/KennethTrecy/abstract_chearmyp_node?style=for-the-badge)
![Commits since latest version](https://img.shields.io/github/commits-since/KennethTrecy/abstract_chearmyp_node/latest?style=for-the-badge)
![Lines of code](https://img.shields.io/tokei/lines/github/KennethTrecy/abstract_chearmyp_node?style=for-the-badge)
![GitHub code size in bytes](https://img.shields.io/github/repo-size/KennethTrecy/abstract_chearmyp_node?style=for-the-badge)

# Abstract Chearmyp Node
An abstraction of Chearmyp nodes.

## Installation
Add it to the dependencies:
```
[dependencies.abstract_chearmyp_node]
git = "https://github.com/KennethTrecy/abstract_chearmyp_node"
tag = "v1.0.0"
```

You may also activate all the features:
```
[dependencies.abstract_chearmyp_node]
git = "https://github.com/KennethTrecy/abstract_chearmyp_node"
tag = "v1.0.0"
features = ["no_std", "assertable_node_kind", "vecdeque_node_queue", "vecdeque_attacher_collection"]
```

## Origin
Some parts of the repository was based from [`filled_bare_metal`] branch of [Feo Template].

## Usage

### Initialization
If you want to contribute, this repository should be initialized to adhere in [Conventional Commits specification] for organize
commits and automated generation of change log.

#### Prerequisites
- [Node.js and NPM]
- [pnpm] (optional)

#### Instructions
By running the command below, all your commits will be linted to follow the [Conventional Commits
specification].
```
$ npm install
```

Or if you have installed [pnpm], run the following command:
```
$ pnpm install
```

To generate the change log automatically, run the command below:
```
$ npx changelogen --from=[tag name or branch name or commit itself] --to=master
```

## Notes

### License
The repository is licensed under [MIT].

### Want to contribute?
Read the [contributing guide] for different ways to contribute in the project.

### Author
Abstract Chearmyp Node was created by Kenneth Trecy Tobias.

[`filled_bare_metal`]: https://github.com/KennethTrecy/feo_template/tree/filled_bare_metal
[Feo Template]: https://github.com/KennethTrecy/feo_template
[MIT]: https://github.com/KennethTrecy/abstract_cheamyp_node/blob/master/LICENSE
[Node.js and NPM]: https://nodejs.org/en/
[pnpm]: https://pnpm.io/installation
[Conventional Commits specification]: https://www.conventionalcommits.org/en/v1.0.0/
[contributing guide]: ./CONTRIBUTING.md
