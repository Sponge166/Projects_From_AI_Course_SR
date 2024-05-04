This is the fourth project for my AI class
The assignment was to write a program that classified text as "interesting" or "un-interesting". 

The qualifications for interesting vs not interesting were up to us, so I chose to classify text based on whether it talks about cats or not.

We Uses a Niave Bayes Classifier which assumes given a string x the probability of and some word w being contained in x given a Classification c (AboutCats or NotAboutCats)
is independent of any other word s being contained in x given c.

thus the probabilty x has a classification c is given by the equation
P(x|c) = P(c) * (P(w0|c) * P(w1|c) * ... * P(wi|c)) for all wi in x.

x is given the classification C for which P(x|C) is maximized
