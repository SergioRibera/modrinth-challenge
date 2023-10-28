## Challenge for Modrinth
In this challenge what I'm taking more time is to decide the dependencies hahahaaha, although this is because I plan to do something a little more interesting what is requested and obviously with scalable and secure code, to really show what I'm capable of, the time it took me to code it can be seen in the delay between each commit, from the initial to the last commit

> [!NOTE]
> [Here](https://github.com/SergioRibera/modrinth-challenge) you have my repository

> [!WARNING]
> I used [this](https://docs.modrinth.com/) API

## How the application works
This application is planned to work in two ways

Through cli:
```sh
modrinth_challenge <slug>
```

Or in an interactive way
```sh
> modrinth_challenge

Welcome to Modrinth!

    :: Type the slugo of the project to search for: _

    :: I found this project
       Name: _
       Description: _
       Downloads: _
       Categories: _, _
```

## Development
You need use the variables store in `.env.example` file


```sh
# Loading .env
> cp .env.example .env
> source .env
> cargo run
```

```sh
# Set variables
> LITCRYPT_ENCRYPT_KEY=<value> API_BASE_URL=<value> MODRINTH_API_KEY=<value> cargo run
```
