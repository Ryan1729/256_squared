# Initial Idea

A game where the interface is entirely text converations where you need to figure out a procedurally generated language.
To cut down on typing, allow picking any previously said word/sentence.

# Spitballing

To start with the language can be just English with a different vocabulary.
Would it be worth it to try making it work even without a language as a proof of concept? Then the fact that you can't use arbitrary English will be annoying.

Thinking about this more abstractly, each person will be a series of state changes you can select from.
Select "Yes" -> "Ah." -> now the character knows something you said.
Select "That would be nice. Thanks." -> Receive item. (Do we want to bother making an inventory?)
Select "Oops." -> Character runs off to talk to other character to tell them what you did.

Okay, if we imagine we have a nice way to define conversation trees and actions associated with them, at runtime, what would we do after that? As in, what specifically would we generate?

We can try to think of evocative things, or we can just start with simple things that will combine in interesting ways. 
We could research conversation puzzles, fetch quests, and that zelda dungeon map diagramming thing.

Let's try imagining a cool moment that we want to be possible and build a situation where that can happen.

Theme idea: You are a blind human that is traveling to different alien inhabited locations. Space stations, etc. You communicate trough a text interface through your environment mech.

Simple conversation ideas: 
"Hey have you seen my smeerp (cat analogue)?"
You go ask around about a smeerp and you slowly realize that one of the sentients you talked to is that other sentient's missing pet. There should be a non-sentient creature as a red herring and/or a sentient with a language that sounds like animal cries.

## Okay, but how?

So if we decide to go with the above idea, then how will the player actually figure out what the words mean? Most games that are similar to this have characters that understand English (or at least the player's language,) and the player learns words through them. It's not uncommon to have the language in question be dead and only a few words known from it at all. So that allows other characters to talk to you and simply tell you about new words. Alternately you can hear or find written words and ask a character about them.

It's tempting to have the player have to figure everything out from first principles, but that is probably unreasonably difficult. Something that would be "on-theme" would be to have a damaged or otherwise unreliable translator. But something not working for an arbitrary reason my be frustrating to the player. One idea would be to have a translation program that simply has never been exposed to the particular languages you are trying to use. You could have early levels have languages that are similar to ones it knows so you only need to help it along somewhat, and then have it know less and less about them as the game progresses.

This framing device makes the UI we want to present to the user make sense, as well. Specifically, giving the player tools to try different meanings for words and seeing if they make sense, while allowing backtracking.