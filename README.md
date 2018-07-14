This project serves as a toy project with the intention to learn basics of
the Rust programming language. Also, I want to develop an application in this
language which will serve a REAL purpose. The gist of the application is
summed up in the following lines.

Everyone needs to send and/or generate a CV sometimes. This can be a very lengthy
and time-consuming process (I know this from my own experience). But should it
be this way?

Now, there are currently three main options by which you can create your own
CV, each having their cons and pros. As a Linux user, I'm only talking about Linux. 
I don't care for Windows (in this matter, anyway).

| Option                               | Pros                                         | Cons                           |
| ------------------------------------ | :------------------------------------------: | :----------------------------: |
| Online service, such as *https://cvmkr.com* | really easy to setup and maintain | **persistency for the user** - for how long will the information stay on the server and/or will the service be available for free ? ) |
|                                               |                                   | privacy concerns
| Libre Office, Microsoft Word or any other word processor | you choose who receives the data (at least in some way) | word processors are usually **huge** (this might not be a con for the vast majority of people as some kind of office suite is usually already installed on the system) and not specialized for such tasks |
|                                               | offline usage | really tedious sometimes (templates, messing around with the resulting look) |
| Specialized desktop applications | privacy | There are a few applications which do this but are VERY **impractical** to install, let alone use. Most of the time, I did not even manage to run the application because it missed dependencies. As an end user, I really do not want to mess around with that, I'm lazy. It's frustrating. |

**System requirements**

Nonfunctional requirements
- the main language will be Rust (for the reason mentioned in the introductory paragraph)
- use NoSQL database - the data will most probably be different from user to user (instead of NULL values everywhere, 
  use JSON documents instead)
- efficiency is not the key here as the normal user will not use the app every day but only occasionally and only
  for a short period of time (that's the whole point of this application!)

Functional requirements
- the application will work straight out of the box (AppImage, maybe?)
- put emphasis on the ease of use (launch the app, quickly create a CV, possibly generate a PDF and quit)
- no fancy graphics (might also want to create a text-based interface), basic user interface
- CV data created must be portable to other machines and perhaps human-readable


