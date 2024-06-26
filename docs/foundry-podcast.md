Anna Rose (00:00:05):
Welcome to Zero Knowledge. I'm your host, Anna Rose. In this podcast, we will be exploring the latest in zero knowledge research and the decentralized web, as well as new paradigms that promised to change the way we interact and transact online

Anna Rose (00:00:27):
This week Tarun and I catch up with our friend Georgios Konstantopoulos from Paradigm. We chat about his move to become CTO of the VC firm, the team he's built and what they're working on. We then dive into Foundry, a fast portable modular testing toolkit for Ethereum app developers written in Rust. We look at the tooling landscape, why they're making a bet on solidity, the different types of testing that smart contract developers are doing, the open questions about tools like this. That is, what happens when you introduce bridges or multiple EVM chains linked together. Basically, Tarun and I just get to ask Georgios all the questions we have about the project and get his thoughts about the space. Was really fun to catch up. Hope you enjoy. Now, before we kick off, I wanna highlight an upcoming event from the ZK validator. Another project that I work on.

Anna Rose (00:01:13):
So on April 15th in Amsterdam, the ZK Validator is hosting an event called privacy in Cosmos. This is our fourth edition in a series of events that it's has been funded by the cosmos hub. We have a great group of cosmos projects and privacy projects coming together for a full day of talks, workshops, panels. So, anyway, I hope to see you there. The event is free, but spots are limited. So be sure to get your spot right away. We've added the link in the show notes the next week. There's also a ZK summit, but I'm assuming that by the time this airs, the spots are probably sold out to anyone who tried to apply, and didn't get a spot this time around. I'm really sorry about that, but we will be doing more ZK summits in the future. We're hoping to do a ZK hackathon. So I hope to see you at one of our future events. So now I'm gonna let Tanya the podcast producer share a little bit about this week's sponsor.

Tanya (00:02:01):
Today's episode is sponsored by Electric Coin Company. The inventors of Zcash who are building the next generation of zero knowledge tech with its network upgrade five, Zcash will move to the Halo zero knowledge proving system, removing the trusted setup and becoming shielded by default. They believe privacy is essential to delivering on the promise of a thriving and equitable web3. With ZCash and Halo cryptography ECC aims to usher in an era of self sovereignty and economic and creative freedom. Visit electriccoin.co to learn more and explore the relationship between privacy, self sovereignty, and economic and creative freedom. That's electriccoin.co. So thank you again, Electric Coin Company. Now here is Anna and Tarun's interview with Georgios from Paradigm.

Anna Rose (00:02:49):
Today, Tarun and I are chatting with Georgios, the CTO and research partner at Paradigm. Welcome to the show Georgios.

Georgios Konstantopoulos (00:02:55):
Hello, how's it going?

Anna Rose (00:02:57):
Georgios. You have been a guest co-host a couple times back in the day. And this is the first time I actually get to introduce you as the CTO of Paradigm. Tarun's also here with us today. We're gonna be looking at the Foundry project, but I think we've, before we do that, it would be really great to hear what you've been up to. What's new in your world?

Georgios Konstantopoulos (00:03:19):
So what's new. It's been a while since we last talked, I think what's new is that recently I've been more involved in building our internal engineering efforts. 2022 is Paradigm from being, let's say a more research, heavy, like hands on company. We're also extending that to becoming software driven and data driven for all our investment processes. So we've been hiring up on, on all sorts of engineering roles for our portfolio and for our internal projects without diving too much into that. I've recently been spending some more time on open source tooling, such as Foundry, which I think is today's topic.

Tarun (00:04:03):
You know, now, now that you're, you're kind of the grand poobah of the engineering team, you know, kind of what, what are kind of the high level goals, you know, you guys have hired a lot of famous anons on the internet. Okay. So, you know what, what's kind of like the, the overall like theme that you think about when you're kind of building this team out too, because like, you know, it's, I mean, there are a couple other examples, but it is pretty rare when the venture fund builds an engineering team. So what are kind the challenges and benefits of that?

Georgios Konstantopoulos (00:04:33):
I think there's two angles here. One is on the research and one is on the internal engineering front on the research front. A lot of the work that we've been doing has been in the realm of supporting portfolio companies, designing new mechanisms. And in particular for these skill sets, we've been on the lookout for very skilled solidity engineers. Recently, if you see, we hired Televens and @Frankieislost on Twitter, who both of them basically were tweeting a lot about high quality solidity that they were building. And it seemed like a good fit for us. And to be clear, I don't think anon or not matters. Age, not a problem either. We are just interested in the best talent across the board. Now on the internal engineering projects, I think there is a lot of data in the market that is not being used properly. I think that our needs are a bit more unique than what out of the box tooling may provide or out of the box. Tooling may just not be fast enough for cost effective or just simply not flexible. So we're gearing up to basically build that out for ourselves.

Anna Rose (00:05:50):
You've also been doing a lot of, I mean, at least a way that I have seen kind of the research and work you do come out is basically disclosing exploits to teams before the bug gets used for bad purposes. Like a lot of these. Yeah. What, what do you even call these disclosures? Like kind of posts where you reveal, oh, we found a bug and we wanna let people know before something bad happens.

Georgios Konstantopoulos (00:06:15):
I think disclosure is the right term here on our team. There is @samczsun who I think is probably the best hacker on Ethereum and maybe other chains in the future. We'll see where basically Sam is always on the lookout for other projects. Maybe it has some automated detectors set up for identifying interesting hotspots or contracts which may get exploited. And basically anytime there's some alert or every time Sam is like, oh shit, there is something going on. He will contact the teams, building the project, ask for context to the teams, building the project, try to build reproduction of the bug so that the teams can know what the issue is. And then responsibly disclose patch if possible and save the day. And this happened multiple times over the year. Yeah, most notably there was one hack where he saved about 350 million dollars on a sushi swap related contract. And, and more recently he rose to the first rank on the Ethereum security leaderboard for finding critical vulnerabilitis in Go Ethereum.

Anna Rose (00:07:27):
Wow. Some of the people you mentioned having hired kind of anon, are they truly anon to you? Like as a company, do you have to kind of know who they are to be able to, to employ them? Like you're not a DAO, you know,

Georgios Konstantopoulos (00:07:42):
Perhaps, perhaps, perhaps.

Anna Rose (00:07:44):
Okay.

Georgios Konstantopoulos (00:07:45):
We like to know who the people that we work with are, although we are very protective of their identity and we respect everybody's privacy to the utmost.

Anna Rose (00:07:54):
Got it. Today what we really wanna dive into is the Foundry project. Before we do that, I wanted to sort of lay the scene of like where it lives and what is similar to it. Like kind of what came before, what tools would people be familiar with that it aims to kind of like replace or add to.

Georgios Konstantopoulos (00:08:15):
Yeah. Foundry is an Ethereum development tool and more specifically an EVM development tool. So it's not tied specifically to Ethereum. It can be used for Polygon, optimized market room, whatever other EVM chain exists. It is a testing framework in the neighborhood of Truffle, Hardhat, Dapp tools, Brownie, and all the other tools that have been have existed over time. And its core objective is to give you a good testing environment. And why is that important? It is because smart contracts need to be tested properly because if they're not tested properly, edge cases exist. And these edge cases tend to cost people money. And whenever something bad happens, you see headlines with X million dollars got lost into this hack and turns out there's some missing check.

Anna Rose (00:09:09):
Is it often the hacks we were just talking about that. Like Sam, actually, what, how do you say his name again? Say son,

Georgios Konstantopoulos (00:09:16):
samczsun, but there's also a very specific branding page on his website for people writing it.

Anna Rose (00:09:23):
Okay. samczsun, was Sam's those, those exploits that you just described are those, the kinds of things that these type of tools are meant to find, or are they bugs that kind of a for level?

Georgios Konstantopoulos (00:09:36):
These are not tools that automatically detect bugs in any way. These tools typically have a bit more limited scope into what they can find for you. Testing frameworks are there for you to get confidence that your code is supposed to do what you wrote it to do. So when you write some code, you write tests and the tests say, okay, two plus two is expected to equal four. Whereas if two plus two, in some of your functions ended up equaling five, clearly there is a problem. So whenever a developer writes code, they have this very hot feedback loop in their system, which is write the code, build it, tested, debug it, right. And you know, you keep doing that over time. And what's important is for the developer to have this feedback loop very optimized. So you want both the build process and the execution of the tests to be very fast. And this is where Foundry kind of comes in as a tool that covers the gap from pretty use tooling, which traditionally has been a few times slower. And we can dive, dive into that in a bit.

Anna Rose (00:10:43):
Yeah. I mean, can you share with us a couple examples of the previous tooling? Cause I can't, I mean, tooling has been around there's things like truffle, there's Dapptools and I guess Hardhat sort of goes in the same category?

Georgios Konstantopoulos (00:10:55):
For sure. Okay. So right now, according to the latest developer survey that was executed by the Solidity team, Hardhat is the most popular developer tool followed closely by Truffle, then it's Brownie. And then there is, you know, the newer ones like Foundry and Dapptools. So there is a core difference between the two that in Hardhat Brownie and Truffle you write your tests in a different language than what your actual code was. So your code language is typically solidity or maybe Vyper or any of the other new languages, but typically Solidity or Viper. But it turns out that Hardhat and Truffle you write your test in. So in JavaScript or TypeScript, and in Brownie, you write your tests in Python and changing from one language to the other. As a developer, it has a lot of contact switching literally in your mind, like when you're writing something, you're suddenly in one programming style and then you change language in a different programming style.

Georgios Konstantopoulos (00:12:02):
And this has caused a lot of, let's say productivity loss to people. And also in the JavaScript library specifically, there has been a lot of incompatibilities with libraries, there's an infamous big numbered library, for example, for doing maths on very large numbers that people never are able to get it to work because they try to make JavaScript connect to solidity. So dApptools and Foundry, they take a different approach. They say, you know what? We want to write our tests like in every other language, and we're gonna write them in the same language that we're developing our contracts on. If you look at any of the big programming languages Go, C++, Rust, Python, whatever, they all have the tests in the same language. So here you also write test in Solidity, code in Solidity.

Anna Rose (00:12:56):
What are you actually running? Is it like a library? That's testing something? Is it a tool? Is it an application? Yeah. You sort of mentioned these two coding fronts. So what are you doing when you're doing a test?

Georgios Konstantopoulos (00:13:08):
Yeah. Originally how this worked is that somebody had to deploy a Test net node with it was back in the date. It was called test RPC. Then it was replaced by another thing called Ganache. And what these two do is that they give you a Testnet node hosted locally without like any heavy database. And what you do is that you literally send network requests to it saying, deploy my contract, then call this function of my contract, then call this other function of my contract. Then check that the value returned by this contract is this expected value. So a test literally is something that says deploy contract, call function, check some assertion, but in the different languages paradigm, what happens is that you deploy your contracts to a node and you send requests to that node. Whereas the difference here is that you're never deploying your contracts to a node in DApptools and Foundry you're instantiating, a local Ethereum virtual machine, and you're running calls against it in the same process. And this is the core ingredient that gives you a lot of speed boosts.

Anna Rose (00:14:23):
Cool. And when you talked about that node in the previous example, though, would that be like that's a test testnet node, I guess, right?

Georgios Konstantopoulos (00:14:31):
Yes. Yes. A local testnet.

Anna Rose (00:14:33):
Okay. It was also local, but in the Foundry example, it's an instantiation. Why is that different?

Georgios Konstantopoulos (00:14:39):
And node is composed of the EVM some database and then, and a set of end points that a user calls to interact with it. Whereas here we just never instantiate the, the whole RPC layer of talking to the node and just talk to the EVM directly. So it's okay. You're, you're closer to the metal in a way when you're operating yeah. In this way. And by being closer to the metal, you can be more effective and more fast

Anna Rose (00:15:06):
It's faster, I guess.

Georgios Konstantopoulos (00:15:07):
Yeah. Yes. Less layers of abstraction.

Tarun (00:15:10):
How do you kind of view dealing with across chain world in testing? Right. Because you know, as people start building more and more complicated applications across chain, you'll basically need to instant have Foundry run, you know, simulate sort of the multiple virtual machines, some notion of how the bridges work and sort of like, yeah. How what's, what's kind of the way you're thinking about that. Are you thinking about like depth first search of like, just add as many features to the existing thing until it's just like the perfect unicorn or are you thinking like breath first search of like, how do we find the minimal set of features and then expand to as many new things as possible?

Georgios Konstantopoulos (00:15:50):
It's a good question. So be basically, let's say I am, I'm going to test bridge that talks to Ethereum and optimism. How do I test that? There's a couple of approaches here. One is that you think of the local chain as the place where you are testing. So let's say I'm on Ethereum. I have the... I will test just Ethereum side of things. And I will mock the opposite side. I will say, you know, instead of actually deploying the contracts on optimism, I will say in my tests when some call is made to this address insert this response as if it was executed on optimism, but not really, it's never executed on optimism. You just say, you know, play act like it was an optimism response. And this is a common pattern in testing. We call it mocking. It just means, you know, don't run the actual computation.

Georgios Konstantopoulos (00:16:44):
I'll tell you what to return if you receive some kind of inputs. So in this world, the multichain testing thing is not a problem because you just always say give me the response that you expect. Now, if you want to properly integration test against both chains, it gets a bit more complex. And the reason why is because in the full node example that I said before what you do is that you'll have one full node, another full node, and you would get state from both from both of them and run your tests against them locally. However, here is a bit more complex because you have instantiated an EVM with, with Ethereum mainnet state, and then you also need to somehow get optimism state. So how are you going to interleave these two in a nice way. We have some ideas around how we can do this. We have not yet implemented any of this because it's a feature that nobody has asked for yet. So to answer the original question, I think we're going for basically get the best development experience on one chain. And then we'll see if we expand and how, because right now there's a lot of gains to be made on just the single, the single use case.

Tarun (00:18:04):
It, it, it clearly has caught on, you know, people's, you know, people have started switching to it because, or, or sort of maybe augmenting to some extent other tools because of its speed. So I guess that, that sort of was, was the natural question of like, do you just double down on that? It sounds like that's kind of your plan.

Georgios Konstantopoulos (00:18:23):
If you compare performance one person recently in our support channels, they said that they had the brownie repository where tests were running in some like 75 seconds without any, some form of advanced testing, which we'll touch on in a bit. And in Foundry, they were running in like 12 seconds or 13 seconds. So a five X improvement is a lot now maker DAOs operations team, the ones that are deploying all the governance proposals. They had a test suite, which when they were running it on CI on GitHub actions, it would take about 20 minutes to complete. And if you, I think one or two months ago, when they were testing with Foundry, they run it in 12 seconds, that's a hundred X faster. And if you're developing this time difference, it literally like is money for your business. Like you're paying your developers to write code. So how we see Foundry playing in, let's say Paradigm strategic positioning is that this is the, one of the most scalable forms of value add that we can add to the portfolio, literally saving your developers' time.

Tarun (00:19:30):
One interesting thing about this is like, if we look at the initial Foundry team, it's a very multi what it, there are people working at multiple funds on it. Like, you know, if I, if I, if I, I think about the initial team as like you Brock transmissions, I guess to some extent, actually, who would you call the Genesis team? And it's kind of interesting that you have multiple people multiple funds working together on building this tooling. That's like pretty much pure public good.

Georgios Konstantopoulos (00:19:57):
Right? So how it started is that we had a library called Ethers RS, which is a toy project. I had started two years ago for MEV traders to have better infrastructure, which is right now used by everyone, basically that does MEV trading. And then one guy called Matt Sage had started to contribute to it. He is a Rust developer who was working with ChainSafe and is now working with us to improve Foundry. And then I had started working on Foundry where, which at the time we were calling it dAppTools Arrest, he was one of the first people to contribute to it. So it started off very open source toy project. Let's see what we can do. Almost thinking alongside of the meme, let's rewrite things in Rust, like for fun. And then it turned out that it was quite good. So first it was say, then I think it was Brock Elmore from Nascent that started to contribute who is also like one of the top contributors to the project. Brock is mostly known, I think, in the industry for Yams, I believe among other things. Yeah, yeah, yeah, yeah. There was a large, large incident back in DeFi summer for the people who were around.

Tarun (00:21:12):
When you think about it, that Yams incident isn't even that big anymore. Like that would be like a blip.

Georgios Konstantopoulos (00:21:17):
Right? It is true. It is true, but it kind of underlines how much things changed the last almost two years. Like it's, it's DeFi summer anniversary or months. Wow. And the, and the third, and, and I think the third person that has been like very, very valuable to the project has been Oliver Nordburg, who was one of the oldest. I think he was the first hire at anon who was sort of freelancing for a time. And then he started again also spending more time with us on Foundry. So basically the project sort of started very, very organic, very let's see what happens. Some open source contributors appeared. And the once let's say that we felt their time spent on the project would be very valuable and would be a shame if they would be spending time on something else. Or I, I would feel very sad personally, if I didn't have Matt, Oliver or Brock, like we working on this,ubasically it, it was a nice way for us like to get them to contribute more.

Georgios Konstantopoulos (00:22:16):
And Brock I believe that basically all of their projects that they do at Nascent they're using Foundry for development. So it kind of makes sense for them to also contribute to it, even though we're not involved in a more formal relationship. So I would say that is the core group. And then you will see a lot of, a lot of people making periodic contributions, too many count. I think there's about 75 or 80 people on the GitHub report that are actively contributing. And there is about 500 like actual, very technical people in the channels, posting feedback, posting support questions, improving the project. So it's not like, you know, it's a 500 or 1000 people chat where people are asking when talking, when talking

Anna Rose (00:22:59):
This currently it lives within the paradigm repo though, I guess. Right. It's still within your orgs.

Georgios Konstantopoulos (00:23:05):
It is under my personalb GitHub right now.

Anna Rose (00:23:07):
I, I, I'm kind of curious, like, does this live within the paradigm like repository or is it not, is it open? Like how do you actually keep a project like this? Where does it live? What are the licenses and stuff associated with it?

Georgios Konstantopoulos (00:23:21):
The project, the GitHub project lives under my personal GitHub under gakonst/foundry and the license, a very permissive permissive one it's Apache MIT it's free for anyone to for contribute to it. Well, I, I don't think we really believe in the value of the project being in kind of putting a walled garden around it and trying to say, you know, this is our property. Don't touch it. The whole value of it comes from people coming in, contributing, talking about it, working with it. So, you know, contributions and forking, the project are actually a metric that I think makes it look successful rather than the opposite.

Anna Rose (00:24:02):
Cool. How do you see sort of maintenance for a project like going forward? Do you see this being something that you are going to champion for a long time? Do you ever imagine this kind of being, I don't know, taken on by another org?

Tarun (00:24:14):
Exit to the community?

Georgios Konstantopoulos (00:24:16):
I think the exit to the community part, you know, the biggest problem with open source is always funding. So when you have an open source project, you need to figure out how to have a sustainable model for it. I think right now we are happy to provide and commit time. And frankly like some capital to it to keep it moving forward. As long as it is good for the industry, because we contribute back to the industry because we like the industry. So as long as we think that this is a project that is valuable for crypto, we will, we will contribute to it in some capacity. Now, whether it is me spending all my time or it or somebody else, I do not know, but I think we really want this to succeed right now.

Anna Rose (00:25:03):
Who uses this product, you know, it it's for testing solidity code. Cool. But like, do you actually do, do you have it built, like, are there projects that have it like kind of built into their pipeline? Like what, what does it mean for this to be used, I guess is a question.

Georgios Konstantopoulos (00:25:19):
I think the biggest library contract that uses it right now is probably Soulmate. Soulmate is a project built by transmissions 11, which is "modern opinionated and gas optimized smart contracts for Ethereum, they're basically, it's a set of smart contracts. That's a competitor to open Zeppelin contracts. And it uses Foundry for testing. Maker DAO uses it for their operation. There is a, and then you will see a lot of like smaller bottom up, let's say efforts from individual developers or hobby projects. So basically I think that right now, Foundry has crossed the chasm for new developers or for, you know, people hacking basically it seems like they are choosing Foundry, but the opposite it, you know, top down big companies still adopting. It is still ongoing because primarily people have all their testing infrastructure already written in a different setup. So it's harder to get them to migrate.

Tarun (00:26:21):
And testing is sticky. No one wants to replace it because it like you, you know, you have the, like who watches the, when you, when you change your tests, you have to make sure that your new tests don't break the old tests and they're backwards compatible dot, dot, dot, dot dot, right. And like, that's a lot of work that can be a lot of like huge amount of work.

Georgios Konstantopoulos (00:26:40):
I agree. So basically there, there's a few things we should think of tests as the glue that keeps our code from falling apart. So when you're changing your tests, you want your, your glue to be the same as it was before, because otherwise your code might fall apart and you don't want that. So how we've built a project is that it's very easy to run alongside whatever else you're already doing. So maybe if you have a bunch of Hard Hat tests and you don't want to remove them, but there is some areas of your code that you want to extend coverage for, or maybe some of your test is very slow and you want to make it faster. You can do some kind of incremental change where you learn, you run Foundry alongside Hardhat and you make incremental changes. And there are basically example repos for every framework. Now there is like Truffle Foundry. There is Hardhat Foundry. There is Brownie Foundry. Yeah. So just to recap, bottom up smaller projects, I think we're in a good spot top down there is, let's say some friction because of inertia, but I think that projects are very openminded and I think we'll see a few high profile projects in the coming months really is their new versions tested with Foundry. And we use it for all of our internal projects too.

Tarun (00:28:01):
And the history of like CI tools in, in like, you know, normal sort of software stuff has always had this thing where like big companies will always have like a pilot project. That's like a new project that's isolated from the rest of the code base. And then they'll try the new CI tool on that. And then if it goes well and it's faster, provides whatever benefits that it's supposed to, then they like slowly expand to like the rest, their projects. I don't expect that this part of the development cycle to be different. There's like new projects within big orgs will probably start with that. And then, you know, Osmosis after that.

Georgios Konstantopoulos (00:28:36):
Yeah, I think so.

Anna Rose (00:28:37):
You keep using the term tests, but there's also like the term fuzzing. Are they the same thing?

Georgios Konstantopoulos (00:28:44):
If a good question. So think of it like that. A test says for inputs two and two, the output is going to be four. Whereas when you are fuzzing or testing for properties, to be more specific, you're saying that like calling the function add, on A and B always going to be equal to A plus B. So in a way, instead of testing for a specific test case, because testing just for that specific test case might not touch on all edge cases. Instead you specify your test to be generic over your inputs and you run in test for thousands of inputs. So instead of saying, you know, two plus two equals four, I will say one plus one equals two, one plus two equals three, and you keep doing it for many, many, many, many inputs. And the idea is that when you're writing your tests and you test them over many, many, many inputs, you can get more let's say confidence that your code is safe because maybe let's say for an example, you have a division. So A over B and you know, it can be five over two, or it can be 5 over 0 and 5 over 0 obviously does not work. So you want your code to be catching this edge case. Okay. But if you only wrote a test that test 5 over 2, you wouldn't be able to catch it. So you need a way to be able to run your tests over as many inputs as possible to catch as many edge cases as possible. Does that make sense?

Anna Rose (00:30:21):
Yeah, I think so. But I mean like Foundry, you could also use that for fuzzing, I assume.

Georgios Konstantopoulos (00:30:27):
Yes. Yes. So basically, so this is property based testing, which I admit we use, we call it fuzzing. But it's not, you know, for the formal verification people, this is not exactly the right term, but it's shorter to say fuzzing in set of property based testing. So apologies there. So fuzzing in Foundry is supported very easily by just adding an extra parameter to your test. And instead of saying, you know, X equals 2, you run your test over X. So instead of having no argument in your function, you say open parenthesis, a X close bar antithesis and your type. And anytime you just run your tests, your fast tests also run next to your unit tests. So tests without parameters are called unit tests, test with parameters. We call them fast tests. Now, the reason why this is very good is because you don't need to know anything about fuzzing.

Georgios Konstantopoulos (00:31:22):
You just say, run my tests. You are the parameter. And conceptually for you as a developer, you just say, okay, my test is going to run over many inputs. Fantastic. But before that, and before DApp tools also did that, which is a design that we took from there. People had to install extra software to run alongside it. And for developers learning to use new software, installing it, maintaining it, putting it in the, into their CI, it's a pain and nobody wants to do it. And so for example Trail of Bits is kid now, a great, amazing tool that people should be using. But it turns out that people don't bother to download it and use it because the installation process has some steps or because they need to run two software like to, to different commands, to get their tests running.

Anna Rose (00:32:14):
This is like dependencies, I guess, right? This is these build dependencies.

Georgios Konstantopoulos (00:32:19):
It is both requiring to have, you know, you need to install another thing. So think of it like that. As a developer, you want to run one command, run, build, run, test, we're done, you know, install, build, test, that's it. Whereas here you need to install, build test for two things and you have a different config file for the other thing, it ends up having a lot of fragmentation. And especially if you start to have many repos, many projects, many developers that are not used to all of this separate tooling, it is a lot of overhead for you. So an explicit goal of Foundry is to reduce the mental overhead that the developer needs to have for doing their job at their, at their peak performance. So what we say is that instead of requiring that you install a bunch of different stuff we maybe reimplement some of the logic that other people have in different languages.

Georgios Konstantopoulos (00:33:15):
In our favorite permanent language, we bundle it all together and we give you and one liner script to install it. So you install it, you build your code and you run your tests. And these tests support fuzzing, and they support a lot of other good things. But basically the goal is to reduce friction as much as possible. Like, for example, if you, if you look up how many projects use a kid, right now, It's not at the point where everyone like is open to using it as much. And I'd love to if it were the case, but it just isn't, it is still considered the power user tool.

Anna Rose (00:33:50):
Is it similar? I mean, I know you were, and I guess still are a fan of Rust and like, that's, I remember you being kind of like very, very, very involved in that community when I met you. Are you still?

Georgios Konstantopoulos (00:34:04):
Yes. So when we were meeting it was when we were talking about I think it was almost a couple of years ago when I was working with Celo and Dalio on a bunch of the cryptography stack. So Foundry is also written in Rust. Okay. and in particular it's written in Rust two main reasons, one being performance, because rust is just very, very, very fast. And that's our goal here. And two rust is very portable and produces very small binaries. Now, what this means is that I can have a process which produces a program that is few megabytes, like 5, 6, 7 megabytes, which has all the functionalities of Foundry for you. And this is very powerful because if you, if you use HardHat, for example, and you go to your node modules folder. So this is the standard folder that every developer has when they use Hardhat and you go under the Hardhat directory, there's like tens of megabytes of data like that need to be stored.

Georgios Konstantopoulos (00:35:04):
Or if you have an incompatible, nodeJS version, maybe you need to, to roll back to it. There's a lot of ecosystem related things that are just hidden complexity for the developer that you just don't have when you just have a tiny Rust binary program that you just download, you, put it in your, your path and just run it. So the benefit here of Rust us is cross platform. It works on windows, on Linux, on Mac, on M1 Mac on arm Linux. And it works. And it's very easy to install. You can literally install it in two seconds.

Anna Rose (00:35:41):
Cool. Actually, one of the reasons I was asking the question about Rust is also like in rust, there's these automated tests that are always happening. And I'm just wondering if like fuzzing and these kinds of solidity tests, are they at all similar? Is it sort of like trying to recreate that or is it a completely different and I'm a little off the park. I don't know.

Georgios Konstantopoulos (00:36:00):
I think philosophically, we are designing the testing experience to look like Rusts in a way, like separating your contracts and your tests, like in a certain way following certain patterns and so on. And we indeed use a Rust library for fuzzing because you know, we support fuzzing in solidity, but at the lower level, this ends up being some Ethereum byte code that gets fed to a Rust fuzzer. Okay. So your, your fuzzing is only as good as your fuzzer and we have identified certain points that we don't actually like fuzzer and we're modifying it and so on. So there are dependencies to your point earlier, but we're trying to minimize them. And especially, we want to have very minimum run time dependencies, which means like you download it and you don't need to have open as a sale installed, for example, which is a, a dependency that exists on many systems. For example,

Anna Rose (00:37:02):
I kind of wanna go back to this, this idea that Tarun and you had brought up about the different EVMS and like sort of the like cross chain. But I actually wondered if, as I understand it, this is really built for, and by the way, I never know how to call this like EVM for Ethereum, like the actual mainnet EVM, but I'm saying the Ethereum Ethereum virtual machine, I realize when I say Ethereum, I don't know how you call it Georgios. What do you call the, what do you call the original?

Georgios Konstantopoulos (00:37:28):
They're all, they're all the same virtual machine. They're all the same virtual machine, every EVM chain that exists uses the same underlying EVMs.

Anna Rose (00:37:38):
Are they not, not different though? Are they not altered in certain cases?

Tarun (00:37:42):
To be fair, to be fair. Don't, don't, don't forget this. Gnosis chain exploit yesterday showed that they're not truly you know, one to one where like extra functions can suddenly get executed posts. There was, there is some difference for the, for instance, for token standards, right? Like people have different ways of redeploying things automatically and stuff like.

Georgios Konstantopoulos (00:38:01):
That. Oh, okay. Sure. But the actual, so for people's context yesterday, Tuesday, 15th, there was an exploit on Gnosis, one of Gnosis chains ERC 20 tokens, because the default ERC 20 factory from the token bridge is an actual ERC 20. It is ERC 20 plus an unsafe extension. And this unsafe extension was abused and people lost money. And, and that was very bad, but it's not like the actual EVM running under the hood is different.

Tarun (00:38:38):
Okay. Yeah, yeah. Yeah. I, I, I just wanna make that sort of the, one of the reasons I wanna bring that up is that that makes the design space for a tool like this, right? Like an IDE tool that's like multi almost, you know, can support all of these things. It has to be somewhat aware of that at those differences. Right. And like keeping, keeping a list of those, let alone, if you go to some virtual machine with like a different, you know, ABI, but you know, my, the question is like, when you're designing these things and you're thinking about bridges, how do you think about sort of these types of attack vectors and like making tests for them and making it easy for users to kind of find these types of things? Because I, I, in, in a cross chain world, we can only expect these attacks from synthetic assets or like these like sort of implied wrapped assets to only grow. Right? Like, and, and I feel like testing those is like going to be like tantamount in, in the future.

Georgios Konstantopoulos (00:39:32):
I think it's what we talked about in the beginning, in the ... chain world, we would either need to chain A, would need to know that the contracts on chain B are deployed in a certain way. So it is up to the developer, I think is the kind of cop out answer. If you will. You just need to like give enough documentation the developer, you know, be careful this might happen. I don't know that it's possible for us to warn against these kinds of things. It feels candidly. It almost feels a bit out of scope in a way, because I can deploy my bridge and I can add whatever code I want.

Tarun (00:40:09):
Do, do you think you'll have sort of like a module system, because like, one thing you could imagine is that say like each layer 2, or, or another EVM chain, they write their own sort of like modules that you can plug in that like automatically apply. You know, you have the same piece of code for the ETH test and the Arbitrum test, but you're when you run the Arbitrum thing, you run the Arbitrum module, which also adds in like some built-in suite that does the mapping, like, do, do you think that will, there'll be like an extension layer that people can write plugins like that? Because I actually think that might end up being a middle ground.

Georgios Konstantopoulos (00:40:43):
It's a great question. You know, plugins sound great until when you try to implement them, they introduce a lot of like smell. I feel in your code, like you need to allow for the plugin to hook into every piece of your execution and this, in my experience, you know, most developers don't know what the hell they're doing. So it's better that you just make a bunch of choices for them and say, this is the tool it's opinionated use it, that's it. And exposing a very, very, very flexible plugin system might not be the best way to do that for this specific use case. I, I, I can see it working, but if that use case were popular enough, I think I would prefer integrating it natively because plugins, again, you need to, you need to fetch them their runtime dependency and so on.

Tarun (00:41:37):
Right. But in a world where there's like 5,000 EVM, I mean, there's probably not gonna be 5,000, but, but in the world where there's like hundreds of EVM chains, let's say

Anna Rose (00:41:46):
Possible.

Georgios Konstantopoulos (00:41:47):
I think we're open minded to that being the case. And we reevaluate when, when that ends up being the case in a way it feels like the, the, the scarce unit here is time and people's kind of focus. And in order to ship something that does a zero to one or a one to a hundred improvement on the state of Ethereum tooling, we need to be very laser focused. So I think right now our core focus is we have a very fast test runner. We run it across all re posters that we see, we profile it. We see what is the slowest, and we try to reach something like the theoretical maximum of capacity we can get. So we're going to keep improving the speed of the test. We're going to keep adding more features for people to be able to explore more test cases.

Georgios Konstantopoulos (00:42:38):
We're going to be adding coverage. We're going to be adding the formatter. We're going to be making compilation parallel. Basically the idea is that want you to ... Run like test and like in two seconds, like everything is done instead of it being 20 or 30 or 2 minutes or whatever, whereas the whole, the multi chain component in a way right now, at least to me, it feels like a distraction because it, conceptually is in the realm of integration testing and not so much as unit testing, it's like testing a system like a large system of like two different smart contract suites. Whereas here you're specifically testing one smart contract suite and the whole crossing, the boundary part feels a bit out of scope.

Tarun (00:43:26):
Right, right, right. I, I, I mean, like, I think, you know, when I think about products like this you know, probably the most successful version of, of kind of like a development testing environment like this arguably is jet brains. And it took them a long time before they even added kind of like this type of stuff. Right. It took like 10 years before they added this type of stuff. So I, I'm just more asking, you know, kind of on a vision level, not just kind of like a, like, right, right. Two year level, come on. You ask other people are vision as an investor. So I'm allowed to ask you here on a vision level, like where do you see the like 10 year dream?

Georgios Konstantopoulos (00:44:03):
I think, I think the vision level is that basically a developer installs one tool and that tool covers all their needs. So if that need is indeed, I need to integration test across chain will add it. Hmm. Yeah. I think, I think that is definitely the case, but basically, you know, I'm thinking it is kind of shocking how we're like six years into Ethereum development. There's like a lot of experts that know how everything works and they have very hyper optimized workflows, but for, let's say for the rest of us it is a bit hard for people like to do very advanced workflows without being very like multiyear experience developers. And just to say the obvious if the state of tooling, Ethereum or any kind of EVM chain is this way, imagine how there's a non EVM environments. So a lot of the features that exist here, like testing against mainnet state or outputting cold traces would show how every sub call the gas cost, the inputs, the outputs, everything and interactive debugger of these things. You will not find them in other kind of programming environments. So it, it sort of paints a picture around how poorly developed tooling and how much, how big of an opportunity there is to make things better in the tooling environment for any kind of blockchain

Tarun (00:45:30):
Yeah. Was gonna actually get to that. Like, do you, do you envision supporting non Ethereum run times? Let's say.

Georgios Konstantopoulos (00:45:39):
Again. Good question. I think the thing that we are, that we are considering is Starknet, we don't have anything built right now, but the way the system works it's very cleanly abstracted. It is compiler EVM test test runner. So the test runner is abstract over the virtual machine or the run time, and then you need a compiler and a VM. So I have identified the specific Rust repository for a Cairo VM, which Cairo is the runtime for startnet, the ZK rollup network built by StarkWare. So the idea would be we plug the Cairo rust VM on Foundry. We plug the Cairo compiler on Foundry and boom Foundry supports a new system. Now, the reason why this is possible is because the semantics of Starknet are very similar to Ethereum. There are accounts, there are contracts, contracts live at a certain address.

Georgios Konstantopoulos (00:46:44):
You call the contracts by sending some HEX data to some address, and the semantics are very similar. Whereas if you compare that, let's say that if we want to support Solana where the leading development framework is anchored by great developer and friend Armani Ferrante, it is much harder. And so basically I can see us. I think we will support StarkNet, like later this year, or whenever we have time, I can see us expanding to like other kind of languages that follow a similar account model as a Ethereum or StarkNet, but kind of expanding to the Solana style account model or the, the input, the input accounts for each transaction model. It, it feels like it breaks the abstraction. So we cannot really touch it without building a new thing. And just to fill in on what I said before Paradigm is an investor in StarkWare and we've been long time supporters

Anna Rose (00:47:42):
You just mentioned. Starknet and like, I, I was gonna ask you about stuff like ZK EVM, where they would do like solidity compiles to something else. Does that actually change your test? Are there like significant changes that need to be made to that framework?

Georgios Konstantopoulos (00:47:58):
So there's multiple ways to do ZK EVM and to be clear, I'm, I'm low conviction on how these will actually work because there is, there's a whole kind of works that you're opening now on how, on how complexity of generating proofs, where these goes. But basically the two ways to do it is either via trans compilation, which is what MatterLabs is doing.

Anna Rose (00:48:22):
That's like solidity that compiles down to something else. Right. It's not the EVM opcodes. Yeah. Okay.

Georgios Konstantopoulos (00:48:28):
Right. So you have, Solidity in compile to something that's not really VM and it requires a custom compiler and so on, and there's the other approach, which I think is what Scroll Tech is doing

Anna Rose (00:48:40):
And actually Jordi, over at Hermez as well. They're also looking at this like yeah. Where it's like the op codes are actually the same.

Georgios Konstantopoulos (00:48:47):
Right. Exactly. So you, you run, you run exactly the same computation on, on Ethereum, on like on a geth node. And then you take the execution trace from a transaction and you make a proof out of it. So basically in, in that way, the ZK part of the AVM almost runs as a side guard as a side guard process. It's nice because it's modular in a way, you just your Ethereum node that does things and you just generate proofs on the side, which means that this design is compatible with Foundry.

Anna Rose (00:49:18):
The second one, you mean like the,

Georgios Konstantopoulos (00:49:20):
The second design. Yes.

Anna Rose (00:49:21):
Okay. Okay. Got it.

Georgios Konstantopoulos (00:49:22):
It. Yes. So the second design seems compatible because it just there's no modification, but I think the proving process for that going to be very expensive. Okay. Now, for the, for the Matter Labs trans compilation, or like, you know, different compiler approach, one could say that if the compilation process is sound or like maps, semantically one to one, like from salt and ZK solc, solc being the solidity compiler you could say, you know, run Foundry tests against your solidity tests without compiling to the ZK, to the ZK bytecode. And if, if the trans violation, or if the, if the different compiler maps one to one and your tests pass in solidity, then you would expect that your tests would also pass in the ZK context.

Anna Rose (00:50:14):
Got it.

Georgios Konstantopoulos (00:50:14):
Uh and I believe that's how they're doing it. Like basically for doing it with Hardhat. I don't think that Hardhat can actually like interpret the ZK bytecode.

Anna Rose (00:50:24):
I know this is probably outside of the scope of Foundry, but like, are there tests for the ZK part of ZK Rollups like, are there, is there any equivalent in right the minute you get into that other level of cryptography?

Georgios Konstantopoulos (00:50:39):
So for specifically for Cairo in StarkNet for example, yes, there are testing frameworks where you write your tests either in Python or in JavaScript or in Cairo. So we can see how, like the ecosystem of Ethereum has kind of speed run itself into reappearing over there. And yeah, how it works is that they have a Cairo VM, it runs, maybe it runs a node. Maybe it runs in the same process, same things that we talked about in the start of the call and you write your tests and they interact with the VM and they give you an output of how they worked. Again, this is because the semantics map one to one, it's very easy, like to build the new primitives based on how it was done elsewhere on how tests were in other systems. I'm less, I'm lower clarity of thought. So, I mean, what are the other approaches like the ZK EVM, we talked about it the Aleo approach. I'm not sure how Leo tests look like these days.

Tarun (00:51:43):
Yeah. I, I, I think one of the interesting things is like things that maybe it may, it might be like a good future episode is like to basically just get people from three different places. Like maybe it's like one cosmos project, like Henry, one Aleo project, and Georgios, we have like a, like what, you know, what is, what is this, the framework? Because my, my what, from whatever little I've, you know, dumpster diving in the code bases I've done, everyone seems to have very bespoke tests for the pure cryptography side of things, especially the ZK side. Like, it seems like it's very, it doesn't seem like there's like some like, Hey, here are the, like, here's like the can set of tests because like all the, all the different Snark mechanisms are just so different. And like, you have to test very different things, right?

Tarun (00:52:29):
Like for instance, if you have a really big group, then like, you probably have to do like fuzzing randomness testing, where, whereas if you have really small groups, maybe you can actually just like iterate through everything. Like there's stuff like that, where like, there's a lot of like weird nuance to each system that I just don't think there's like some established battery, but like what, what do you think there will be actually that that's maybe maybe a better question is like, do you think like, say in 10 years, there'll just be like the set of like default ZK tests. Like every proving system that's implemented in code has to like pass this like schema of tests. Of course they, they they'll get written and specialized to their particular implementation, but like, there's sort of like a, a, you know, in machine learning there were always, you know, there's these like they're kind of, I guess now kind of gosh, or people don't even care about them anymore because they're just so integrated into the testing framework, but like ImageNet, CFAR stuff like that. There are these like canned data sets where everyone knows what the best possible output is. And like every single new time, there's a new framework. Like you basically have to prove that you can achieve like state of the art and like up to the same error and like bitwise, compatability, depending, depending on the architecture, you're running it on as like there there's some set of tests that's true across all of sort of machine learning stuff. And like some curious if you think that'll happen here.

Georgios Konstantopoulos (00:53:57):
I haven't thought about this too deeply. So maybe the answer will be not satisfactory. I think a good way to get, let's say compatibility across frameworks and whatnot is to define your tests as JSON tests. Or you say these inputs should give me these outputs. You put this in a file and you just say, you know, this is the test suite, no matter what you do, this is what your test framework must run. So this is how, for example, you run Ethereum consensus tests. There's a big, big, big file containing a bunch of inputs and a bunch of outputs. And when people are testing out that their EVM implementation is correct, they literally run it across every single one of these inputs. And they check was the output, the expected one independently of the language. So this has been done Python and go in rust in like any language you can expect has, JavaScript like every EVM, that, but probably the most, the most feature proof way.

Tarun (00:54:54):
Yeah, yeah, yeah. That, that makes a lot of sense. Actually also you said Haskell, and for some reason I just realized that the zero knowledge podcast logo looks like the Haskell logo. It's like almost the same designer, like the Haskell Lambda. Yeah. Really. It's like actually kind of crazy.

Anna Rose (00:55:11):
What? that's crazy. That was my, that, that was my idea. I don't, I'm pretty sure I never saw it, but it's funny cuz Fredrick was really like the original podcast co-host was really big into Haskell. So I wonder if that had any, but like I came up with it. That's weird.

Georgios Konstantopoulos (00:55:28):
It, it kind of does it kind of does.

Anna Rose (00:55:31):
Kind of, kind of, you can.

Tarun (00:55:32):
Cut this Henrick, but I'm, I'm just gonna send this yeah, I'm sending, I'm sending in telegram.

Anna Rose (00:55:39):
Oh yeah. The colors look similar. Right. Damn.

Tarun (00:55:43):
I didn't notice that until like, I just like somehow you said Haskell and I was a like looking at the ZK thing and the, I was like, wait a minute. Is this the Haskell logo?

Anna Rose (00:55:52):
That's so funny. No, I mean, yeah. Yeah. I definitely, I mean, I know where I, my influence it was definitely not that, but that is very funny.

Tarun (00:56:02):
Yeah. But I do kind of think if you make like a vision board for the zero knowledge podcast, you should like have that you some that somewhere like merge the, yeah,

Anna Rose (00:56:10):
Sure. I'd be down. We've definitely mentioned it on the show. Like all of these tests are all focused on Solidity code. I mean, obviously it's cool. Like the project is around Solidity, but like, do you think about a replacement to Solidity maybe in where, when you wear a different hat when you're not wearing the Foundry hat, are you also like, you know, working on that, looking at new languages, do you see anything ever coming in the pipeline that might replace it.

Georgios Konstantopoulos (00:56:38):
Contrary to popular like opinion? I kind of like Solidity. I don't mind it. The semantics,

Anna Rose (00:56:45):
How contrarian!

Georgios Konstantopoulos (00:56:48):
You know, like, Solidity it is fine. Honestly, people like to complain about a lot of things. Like the language is not the problem when things go bad. People like to blame the language, but it's like, people just don't know how to right secure code and that happens in every language you will imagine. So, first of all, we're working on making Foundry abstract over the language used. So right now it uses a package called etherssalt. We're going to generalize that to be called etherscompile. So any language can be used for that. And the idea would be that, you know, we can use it to compile VIPA. We can use to compile FA there's like a bunch of like experimental programming languages. And to Tarun's point, this starts to look like a plugin, like system where you can plug like, whatever language you want in the, in the code.

Georgios Konstantopoulos (00:57:42):
And we can make it if we want it, we could make it look like a plugin, or we could just say these are the languages. Now for more exotic languages, we have internally brainstormed something like a Solidity plus plus type thing in the past. But it's unclear what that would do beyond opinionated changes in the syntax. I think with respect to targeting EVM Solidity, solidity is quite good. So the question, I guess, would be, are you thinking about another virtual machine? I think the answer is no. And if you're thinking about other languages that target the EVM, I would say some Syntactic sugar or some syntactic changes to solidity maybe, but I don't know that we really need some kind of like new language or whatever. Mm. It, it does seem like it....

Tarun (00:58:40):
Yeah. I, I will say one thing we should take from like the, the like web development era on the 90s, like why do we still have JavaScript JavaScript? Why did we instead build all these languages around JavaScript to compile, to JavaScript? Exactly well it's because whoever built the first dev tooling and got the highest distribution one and like for better or worse, none of the Wasm stuff has totally found that fit. And like, it could maybe one, but it, it, it, it, sometimes it's a race to build the best dev tools. Whoever has the best dev tools. First, I think that was true for JavaScript,

Georgios Konstantopoulos (00:59:14):
Agree, agree, fully agree. And, you know, people saying, Wasm solves this, haven't tried building something to Wasm. So it's, it's usually a hard thing to get something to work in Wasm. And the most advanced thing that I know that works today is Cosmo wasm, which is the smart contract runtime in cosmos and even them they're trying to make a solidity, like domain specific language, which transpiles the cosmos, which ends up compiling to Wasm. So yeah, I could see, I could, I could see somebody building a small, like transpiler, that transpiles to solidity, which has like nicer syntax that they prefer, but then, you know, you need to audit the transpiler and like all that. So it's, it's less than ideal.

Anna Rose (01:00:02):
Something we haven't yet talked about in the Foundry conversation are the sort of the two components that you've created. I feel like we probably should have done it earlier, but I wanna make sure I get it in. So there's like Forge and Cast. Yeah. What are those two things? How do they map to what you've already discussed?

Georgios Konstantopoulos (01:00:21):
Yeah. So full, full disclaimer, this kind of separation of like calling the project Foundry and having multiple tools. It comes from Dapptools where Dapptools basically had one tool called Dapp, which is the equivalent forge, which is this testing framework. Okay. and they had another tool called Seth, which is equivalent of cast in Foundry, which is the Swiss army knife. And I'll just do zoom in. So Foundry is a toolbox. We're not going to have just fortune cast. We're going to have more tools in the future. What these tools may be. We don't know. It might be for matters, linkers, it might be static analyzers. It can be any kind of tool that makes the software testing process of a, of a developer easier. Whether the tool gets integrated in one tool or as a new tool. It's TBD. So just say foundry is the umbrella project and the tool actual tools inside of it are forge, which forge is the tool that we use for testing, like Truffle, like Hardhat, like Dapptools.

Georgios Konstantopoulos (01:01:27):
And Cast, you should think of it as a tool that you use when you're doing operations. Like you want to call an on-chain an on-chain method and see, you know, what was the value of this storage variable, or you want to submit a transaction so you can use it to like, just do all your orchestration to submit transactions, to query data from the chain. And it's basically what everyone that I know uses, like for debugging things that go wrong. So, you know, you want to check, do I own this token? And you can write cast, die balance of accounts dot ETH and it'll literally give you back things. So we're trying to make it like, almost read like a natural language to some extent, so that it's more user friendly

Tarun (01:02:10):
Slight, slight step, step, back, very high level, boring question. What's the etymology of all of these terms. Cause like what, what's the, you know, cuz like picking a name is quite important.

Georgios Konstantopoulos (01:02:21):
Yeah. So we, we brainstormed a lot in the Foundry chat at the time. The idea that we had was it's Rust. So we want something that's a bit metallic and because it's tools, we thought, okay, metal tools, okay. Looks like a blacksmith style thing. So where does the blacksmith live in the Foundry? So the Foundry is the place where all the tools are and you know, the Forge is the thing that the blacksmith uses to forge his tools. And the cast is like where all the, like the hot liquid is in now, why one was called Forge one, the other was Cast. It doesn't matter. It was like random choice.

Anna Rose (01:03:04):
Okay.

Georgios Konstantopoulos (01:03:04):
Yeah. We were considering doing a node and you know, there's like the idea that we might call it, you know, forge sub node, so forged node, or maybe we make it a standalone when we call it like blade or something like that. Like branding matters a lot. And I, I think like the whole, yeah, exactly.

Tarun (01:03:21):
Each new thing has to keep promulgating the message, the theme. So that's, that's what

Georgios Konstantopoulos (01:03:28):
Exactly. And if you go, for example, to the main documentation page, which the Foundry book you will see that we have like a little banner, which is like this guy inside his foundry with like, like two metal for like, for cast, with liquid metals falling down. Like it's, it's quite nice. Like we're branding matters a lot.

Anna Rose (01:03:49):
It kind of, it kind of echoes Osmosis actually a little bit. I, I see.

Georgios Konstantopoulos (01:03:54):
Yeah, there there's the meme of like the, do that's doing things. Yeah.

Tarun (01:03:58):
But this is true in a lot of open source, like, you know, home brew, like the Mac package manager, like everything is called like Casks or like brew or like tapping a keg. And, and like the whole thing is built around like liquor references. And it might be one of the most used packages by developers in the world. Right. It helps. Right. And, and it's very sticks it sticks in your brain. You like very easily, you have these like mnemonics built in, and it's like very important actually to pick these things carefully.

Georgios Konstantopoulos (01:04:26):
Right. Agreed,

Anna Rose (01:04:28):
Georgios, I think the last time you were actually on the show, we talked about MEV. I think we were actually interviewing the flash bots folks. So what does this Foundry and MEV have any sort of overlap? Do you ever tell test for the impact of MEV in any of these things? Or is it like a separate space?

Georgios Konstantopoulos (01:04:48):
So Forge has a sub command called run, which you can run against maintennet state. So typically when we want to test some kind, when we want to make a proof concept of something such as samczsun breaking and a main net contract he will write a contract and he will call forge run and he will see script passed. And he will be like, oh shit, we have to do something. So similarly, if you're proof of, if you're building proof of concepts for Arbitrage or for liquidations or whatnot, you can use it. Now I have a template repository, which is called Foundry rust template. So the idea here, being that all MEV traders, what they do is that they have some solidity that will talk to some rust or some python or whatever, and they need to deploy the solidity. They need to test the Solidity, and then they need to have bindings to this Solidity, like actual code that makes them talk to that Solidity to use it in their bots. So the way that template is set up, it is set up in a way that you do one command, run your tests around your, or solidity tests, another command run, your MEV bot tests, and another command run your, your actual bot with the bindings from Foundry. So in a way, I mean, this is a tool for testing, so it's not like it makes you money like for MEV trading, but it is, it is built in a way that it can make bot development experience easier.

Anna Rose (01:06:20):
Okay. So like the, I had Dean and Edgar on a few months or weeks ago, and they're building these bots so they could potentially use it, but it's not that it's designed for that. It's just, their bots could be tested with it kind of.

Georgios Konstantopoulos (01:06:34):
Exactly. Exactly. Okay. Exactly. Although in their case specifically, I believe that they hook into go Ethereum directly. They're customizing go Ethereum. So this wouldn't work out there. But yeah, like there are many ways to do it. Like maybe they build a library that go Ethereum calls to like, there's a lot of ways to do this, so potentially.

Anna Rose (01:06:54):
Okay. So a topic we probably don't have time to dig into in this episode, but I did wanna sort of mention quickly here is the idea of ZK hardware, acceleration, Georgios. I know last time I saw you in Lisbon, you were talking about it. Is this something that you are working on? Is this something that you're looking at? I know there's new initiatives in this space around this as well with stuff like ZPrize. So yeah. What are you, what are you doing on that?

Georgios Konstantopoulos (01:07:17):
I think we're very excited about the category. There is many approaches to doing it and I think I'd be glad to, to bring on a guest in a future episode that can cover these topics.

Anna Rose (01:07:30):
Okay. You have an idea for, for a guest. I mean, I think it's worth an episode soon for sure. We, I mean, Tarun, you and I, last year we did do we, that we talked with the supernatural

Tarun (01:07:41):
It's supernatural. Yeah, yeah, yeah. But they're doing, they're doing all sorts of different stuff now,

Anna Rose (01:07:45):
Although they are running some prover stuff, right. Like I think they were working with Aleo.

Tarun (01:07:50):
Yes. They're the ones who caused the like 10,000 block fork in Aleo in the test, in the Testnet as supposedly that's what the discord drama was about. But,

Anna Rose (01:08:01):
But you're saying that there's another group that we should potentially talk to. So cool. I guess we leave that open for a future episode,

Georgios Konstantopoulos (01:08:07):
For sure.

Anna Rose (01:08:08):
I think this has been a really cool kind of look into the project you were working on. You've been working on Georgios and Foundry. Where can people find out about it? We can add links in the show notes and stuff, but I don't know if you wanna point people anywhere.

Georgios Konstantopoulos (01:08:20):
People can go to my GitHub and it's the first pin message or they can, they pin the first pin repository or they can find me on Twitter and it's gonna be the first tweet that they see. So on twitter.com/gakonst

Anna Rose (01:08:37):
All right. Sounds good. Thanks Georgios for coming back on the show once again, and for sharing with us this project.

Georgios Konstantopoulos (01:08:43):
Thank you for having me.

Tarun (01:08:44):
It's always great to have an, have one of the legendary Greeks of crypto back on the show.

Anna Rose (01:08:50):
Very nice. And I wanna say thank you to the podcast producer, Tanya, the podcast editor Henrick, and to our listeners. Thanks for listening.

## Refs

* <https://zeroknowledge.fm/224-2/>
