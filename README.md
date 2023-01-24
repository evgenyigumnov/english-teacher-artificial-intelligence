# English Teacher Artificial Intelligence
English teacher Artificial Intelligence. Chat with AI and learn English. The AI-teacher will fix your mistakes and increase your vocabulary.


## How to use

Fix token in .env file. https://openai.com/api/

```file
OPEN_AI_API_KEY=sk-KJHLKJLKJGLKJGHLKJGLKJGLJKGLKJ
```
Run application
```bash
cargo run
```

## Example of chat


Please enter a topic of the lesson:

```
hotel
```

Words of lesson:
1. Lodging
2. Accommodation
3. Suite
4. Room
5. Reception
6. Concierge
7. Check-in
8. Check-out
9. Amenities
10. Restaurant
11. Bar
12. Pool
13. Spa
14. Gym
15. Valet
16. Bellhop

Do you know all these words? Say: Yes or write unknown words comma separated:

```
bellhop, valet
```


Bellhop

A bellhop is a hotel employee who assists guests with their luggage and other needs. They
are usually the first person a guest sees when they arrive at the hotel.
Example 1: The bellhop carried our luggage to our room in the hotel.
Example 2: "Hey bellhop, why are you so slow? Hurry up and get my bags to my room!"


Valet

Valet is a service offered by some hotels where a guest can have their car parked and
retrieved by a hotel employee.
Example 1: The hotel valet was very helpful in parking our car.
Example 2: "If you can't do your job as a valet in this hotel, then you should find another job."

What type of establishment typically offers a variety of food and beverages for customers
to enjoy?

```
I think it is restraunt.
```
I think it is restaurant. < grammar fixes

What is the process for check-in at a lodging establishment?

```
You need show a passport and student card. Is it correct?
```
You need to show a passport and student card. Is it correct? < grammar fixes

Yes, it is correct that you need to show a passport and student card in order to prove
your identity and student status. Depending on the situation, you may need to provide
additional documents such as a birth certificate or proof of address.

What kind of lodging would you prefer?

```
I prefer room with toulet and shower. If I visit hotel with wife then I prefer one big bad for two.
```

I prefer room with toilet and shower. If I visit hotel with wife then I prefer one big bad for two. < grammar fixes

What is the average wait time at the reception desk?

```
Usually it could take not more 5 minutes.
```
Usually it could take not more than 5 minutes. < grammar fixes

What services does a concierge provide?

```
You could ask a concierge call you at 7:00 am to wake you tommorow.
```
You could ask a concierge to call you at 7:00 am to wake you tomorrow. < grammar fixes

What kind of accommodation can you find when you travel?

```
When I travel I usualy take apartmant in hotel.
```
When I travel I usually take an apartment in a hotel. < grammar fixes

Do you have a swimming pool at your accommodation?

```
No We do not have swimming pool at the hotel.
```
No, we do not have a swimming pool at the hotel. < grammar fixes