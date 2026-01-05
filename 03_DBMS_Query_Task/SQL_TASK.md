# 50 SQL Queries

## Query 1
**Question:** Write a SQL query to find the name and year of the movies. Return movie title, movie release year.

**Query:**
```sql
SELECT mov_title, mov_year FROM movie;
```

**Output:**
| mov_title               | mov_year |
|------------------------|----------|
| vertigo                 | 1958     |
| the innocents           | 1961     |
| lawrence of arabia      | 1962     |
| the deer hunter         | 1978     |
| amadeus                 | 1984     |
| blade runner            | 1982     |
| eyes wide shut          | 1999     |
| the usual suspects      | 1995     |
| chinatown               | 1974     |
| boogie nights           | 1997     |
| annie hall              | 1977     |
| princess mononoke       | 1997     |
| the shawshank redemption| 1994     |
| american beauty         | 1999     |
| titanic                 | 1997     |
| good will hunting       | 1997     |
| deliverance             | 1972     |
| trainspotting           | 1996     |
| the prestige            | 2006     |
| donnie darko            | 2001     |
| slumdog millionaire     | 2008     |
| aliens                  | 1986     |
| beyond the sea          | 2004     |
| avatar                  | 2009     |
| seven samurai           | 1954     |
| spirited away           | 2001     |
| back to the future      | 1985     |
| braveheart              | 1995     |

---

## Query 2
**Question:** Write a SQL query to find when the movie 'american beauty' released. Return movie release year.

**Query:**
```sql
SELECT mov_year FROM movie WHERE mov_title = 'american beauty';
```

**Output:**
| mov_year |
|----------|
| 1999     |

---

## Query 3
**Question:** Write a SQL query to find the movie that was released in 1999. Return movie title.

**Query:**
```sql
SELECT mov_title FROM movie WHERE mov_year = 1999;
```

**Output:**
| mov_title       |
|-----------------|
| eyes wide shut  |
| american beauty |

---

## Query 4
**Question:** Write a SQL query to find those movies, which were released before 1998. Return movie title.

**Query:**
```sql
SELECT mov_title FROM movie WHERE mov_year < 1998;
```

**Output:**
| mov_title                |
|--------------------------|
| vertigo                  |
| the innocents            |
| lawrence of arabia       |
| the deer hunter          |
| amadeus                  |
| blade runner             |
| the usual suspects       |
| chinatown                |
| boogie nights            |
| annie hall               |
| princess mononoke        |
| the shawshank redemption |
| titanic                  |
| good will hunting        |
| deliverance              |
| trainspotting            |
| aliens                   |
| seven samurai            |
| back to the future       |
| braveheart               |

---

## Query 5
**Question:** Write a SQL query to find the name of all reviewers and movies together in a single list.

**Query:**
```sql
SELECT rev_name AS name FROM movie_reviewer 
UNION 
SELECT mov_title FROM movie;
```

**Output:**
| name                     |
|--------------------------|
| righty sock              |
| jack malvern             |
| flagrant baronessa       |
| alec shaw                |
| victor woeltjen          |
| simon wright             |
| neal wruck               |
| paul monks               |
| mike salvati             |
| wesley s. walker         |
| sasha goldshtein         |
| josh cates               |
| krug stillo              |
| scott lebrun             |
| hannah steele            |
| vincent cadena           |
| brandt sponseller        |
| richard adams            |
| vertigo                  |
| the innocents            |
| lawrence of arabia       |
| the deer hunter          |
| amadeus                  |
| blade runner             |
| eyes wide shut           |
| the usual suspects       |
| chinatown                |
| boogie nights            |
| annie hall               |
| princess mononoke        |
| the shawshank redemption |
| american beauty          |
| titanic                  |
| good will hunting        |
| deliverance              |
| trainspotting            |
| the prestige             |
| donnie darko             |
| slumdog millionaire      |
| aliens                   |
| beyond the sea           |
| avatar                   |
| seven samurai            |
| spirited away            |
| back to the future       |
| braveheart               |

---

## Query 6
**Question:** Write a SQL query to find all reviewers who have rated seven or more stars to their rating. Return reviewer name.

**Query:**
```sql
SELECT DISTINCT rev_name 
FROM movie_reviewer 
WHERE rev_id IN (
    SELECT rev_id FROM movie_rating WHERE rev_stars >= 7
);
```

**Output:**
| rev_name           |
|--------------------|
| righty sock        |
| jack malvern       |
| flagrant baronessa |
| victor woeltjen    |
| simon wright       |
| NULL               |
| mike salvati       |
| sasha goldshtein   |
| krug stillo        |
| hannah steele      |
| vincent cadena     |
| brandt sponseller  |

---

## Query 7
**Question:** Write a SQL query to find the movies without any rating. Return movie title.

**Query:**
```sql
SELECT mov_title FROM movie WHERE mov_id NOT IN (SELECT mov_id FROM movie_rating);
```

**Output:**
| mov_title                |
|--------------------------|
| the deer hunter          |
| amadeus                  |
| eyes wide shut           |
| the shawshank redemption |
| deliverance              |
| the prestige             |
| spirited away            |
| back to the future       |
| braveheart               |

---

## Query 8
**Question:** Write a SQL query to find the movies with id 905 or 907 or 917. Return movie title.

**Query:**
```sql
SELECT mov_title FROM movie WHERE mov_id IN (905, 907, 917);
```

**Output:**
| null |
|------|

---

## Query 9
**Question:** Write a SQL query to find the movie titles that contain the word 'boogie nights'. Sort the result-set in ascending order by movie year. Return movie id, movie title and movie release year.

**Query:**
```sql
SELECT mov_id, mov_title, mov_year 
FROM movie 
WHERE mov_title LIKE '%boogie nights%' 
ORDER BY mov_year ASC;
```

**Output:**
| mov_id | mov_title     | mov_year |
|--------|---------------|----------|
| 10     | boogie nights | 1997     |

---

## Query 10
**Question:** Write a SQL query to find those actors with the first name 'woody' and the last name 'allen'. Return actor id.

**Query:**
```sql
SELECT act_id FROM actor WHERE act_fname = 'woody' AND act_lname = 'allen';
```

**Output:**
| act_id |
|--------|
| 11     |
| null   |

---

## Sub Queries

## Query 11
**Question:** Write a SQL query to find the actors who played a role in the movie 'annie hall'. Return all the fields of actor table.

**Query:**
```sql
SELECT * FROM actor 
WHERE act_id IN (
    SELECT act_id FROM movie_cast 
    WHERE mov_id = (
        SELECT mov_id FROM movie WHERE mov_title = 'annie hall'
    )
);
```

**Output:**
| act_id | act_fname | act_lname | act_gender |
|--------|-----------|-----------|------------|
| 11     | woody     | allen     | m          |
| null   | null      | null      | null       |

---

## Query 12
**Question:** Write a SQL query to find the director of a film that cast a role in 'eyes wide shut'. Return director first name, last name.

**Query:**
```sql
SELECT DISTINCT d.dir_fname, d.dir_lname 
FROM director d 
WHERE d.dir_id IN (
    SELECT md.dir_id 
    FROM movie_direction md 
    WHERE md.mov_id = (
        SELECT mov_id FROM movie WHERE mov_title = 'eyes wide shut'
    )
);
```

**Output:**
| dir_fname | dir_lname |
|-----------|-----------|
| stanley   | kubrick   |

---

## Query 13
**Question:** Write a SQL query to find those movies that have been released in countries other than the united kingdom. Return movie title, movie year, movie time, and date of release, releasing country.

**Query:**
```sql
SELECT mov_title, mov_year, mov_time, mov_dt_rel, mov_rel_country 
FROM movie 
WHERE mov_rel_country != 'uk';
```

**Output:**
| mov_title     | mov_year | mov_time | mov_dt_rel | mov_rel_country |
|---------------|----------|----------|------------|-----------------|
| the innocents | 1961     | 100      | 1962-02-19 | sw              |
| annie hall    | 1977     | 93       | 1977-04-20 | usa             |
| seven samurai | 1954     | 207      | 1954-04-26 | jp              |

---

## Query 14
**Question:** Write a SQL query to find for movies whose reviewer is unknown. Return movie title, year, release date, director first name, last name, actor first name, last name.

**Query:**
```sql
SELECT m.mov_title, m.mov_year, m.mov_dt_rel, d.dir_fname, d.dir_lname, a.act_fname, a.act_lname 
FROM movie m, director d, actor a 
WHERE d.dir_id IN (
    SELECT md.dir_id FROM movie_direction md WHERE md.mov_id = m.mov_id
) 
AND a.act_id IN (
    SELECT mc.act_id FROM movie_cast mc WHERE mc.mov_id = m.mov_id
) 
AND m.mov_id NOT IN (
    SELECT mov_id FROM movie_rating
);
```

**Output:**
| mov_title                | mov_year | mov_dt_rel | dir_fname | dir_lname | act_fname | act_lname |
|--------------------------|----------|------------|-----------|-----------|-----------|-----------|
| the deer hunter          | 1978     | 1979-03-08 | michael   | cimino    | robert    | de niro   |
| amadeus                  | 1984     | 1985-01-07 | milos     | forman    | f. murray | abraham   |
| eyes wide shut           | 1999     | null       | stanley   | kubrick   | nicole    | kidman    |
| the shawshank redemption | 1994     | 1995-02-17 | frank     | darabont  | tim       | robbins   |
| deliverance              | 1972     | 1982-10-05 | john      | boorman   | jon       | voight    |

---

## Query 15
**Question:** Write a SQL query to find those movies directed by the director whose first name is woddy and last name is allen. Return movie title.

**Query:**
```sql
SELECT mov_title FROM movie 
WHERE mov_id IN (
    SELECT mov_id FROM movie_direction 
    WHERE dir_id IN (
        SELECT dir_id FROM director 
        WHERE dir_fname = 'woody' AND dir_lname = 'allen'
    )
);
```

**Output:**
| mov_title  |
|------------|
| annie hall |

---

## Query 16
**Question:** Write a SQL query to determine those years in which there was at least one movie that received a rating of at least three stars. Sort the result-set in ascending order by movie year. Return movie year.

**Query:**
```sql
SELECT DISTINCT mov_year 
FROM movie 
WHERE mov_id IN (
    SELECT mov_id FROM movie_rating WHERE rev_stars >= 3
) 
ORDER BY mov_year ASC;
```

**Output:**
| mov_year |
|----------|
| 1954     |
| 1958     |
| 1961     |
| 1962     |
| 1977     |
| 1982     |
| 1986     |
| 1995     |
| 1997     |
| 1999     |
| 2001     |
| 2004     |
| 2008     |
| 2009     |

---

## Query 17
**Question:** Write a SQL query to search for movies that do not have any ratings. Return movie title.

**Query:**
```sql
SELECT mov_title FROM movie WHERE mov_id NOT IN (SELECT mov_id FROM movie_rating);
```

**Output:**
| mov_title                |
|--------------------------|
| the deer hunter          |
| amadeus                  |
| eyes wide shut           |
| the shawshank redemption |
| deliverance              |
| the prestige             |
| spirited away            |
| back to the future       |
| braveheart               |

---

## Query 18
**Question:** Write a SQL query to find those reviewers who have not given a rating to certain films. Return reviewer name.

**Query:**
```sql
SELECT rev_name FROM movie_reviewer 
WHERE rev_id NOT IN (
    SELECT rev_id FROM movie_rating
);
```

**Output:**
| rev_name         |
|------------------|
| alec shaw        |
| wesley s. walker |

---

## Query 19
**Question:** Write a SQL query to find movies that have been reviewed by a reviewer and received a rating. Sort the result-set in ascending order by reviewer name, movie title, review stars. Return reviewer name, movie title, review stars.

**Query:**
```sql
SELECT 
    (SELECT rev_name FROM movie_reviewer WHERE rev_id = mr.rev_id) AS rev_name,
    (SELECT mov_title FROM movie WHERE mov_id = mr.mov_id) AS mov_title,
    mr.rev_stars 
FROM movie_rating mr 
ORDER BY 
    (SELECT rev_name FROM movie_reviewer WHERE rev_id = mr.rev_id),
    (SELECT mov_title FROM movie WHERE mov_id = mr.mov_id),
    mr.rev_stars;
```

**Output:**
| rev_name           | mov_title           | rev_stars |
|--------------------|---------------------|-----------|
| null               | blade runner        | 8.2       |
| null               | princess mononoke   | 8.4       |
| brandt sponseller  | aliens              | 8.4       |
| flagrant baronessa | lawrence of arabia  | 8.3       |
| hannah steele      | donnie darko        | 8.1       |
| jack malvern       | the innocents       | 7.9       |
| josh cates         | good will hunting   | 4.0       |
| krug stillo        | seven samurai       | 7.7       |
| mike salvati       | annie hall          | 8.1       |
| neal wruck         | chinatown           | null      |
| paul monks         | boogie nights       | 3.0       |
| richard adams      | beyond the sea      | 6.7       |
| righty sock        | titanic             | 7.7       |
| righty sock        | vertigo             | 8.4       |
| sasha goldshtein   | american beauty     | 7.0       |
| scott lebrun       | trainspotting       | null      |
| simon wright       | the usual suspects  | 8.6       |
| victor woeltjen    | avatar              | 7.3       |
| vincent cadena     | slumdog millionaire | 8.0       |

---

## Query 20
**Question:** Write a SQL query to find movies that have been reviewed by a reviewer and received a rating. Group the result set on reviewer's name, movie title. Return reviewer's name, movie title.

**Query:**
```sql
SELECT DISTINCT 
    (SELECT rev_name FROM movie_reviewer WHERE rev_id = mr.rev_id) AS rev_name,
    (SELECT mov_title FROM movie WHERE mov_id = mr.mov_id) AS mov_title 
FROM movie_rating mr;
```

**Output:**
| rev_name           | mov_title           |
|--------------------|---------------------|
| righty sock        | vertigo             |
| jack malvern       | the innocents       |
| flagrant baronessa | lawrence of arabia  |
| null               | blade runner        |
| victor woeltjen    | avatar              |
| simon wright       | the usual suspects  |
| neal wruck         | chinatown           |
| paul monks         | boogie nights       |
| mike salvati       | annie hall          |
| null               | princess mononoke   |
| sasha goldshtein   | american beauty     |
| righty sock        | titanic             |
| josh cates         | good will hunting   |
| krug stillo        | seven samurai       |
| scott lebrun       | trainspotting       |
| hannah steele      | donnie darko        |
| vincent cadena     | slumdog millionaire |
| brandt sponseller  | aliens              |
| richard adams      | beyond the sea      |

---

## Query 21
**Question:** Write a SQL query to find those movies, which have received highest number of stars. Group the result set on movie title and sorts the result-set in ascending order by movie title. Return movie title and maximum number of review stars.

**Query:**
```sql
SELECT 
    (SELECT mov_title FROM movie WHERE mov_id = mr.mov_id) AS mov_title,
    MAX(mr.rev_stars) AS max_stars 
FROM movie_rating mr 
GROUP BY mr.mov_id 
ORDER BY (SELECT mov_title FROM movie WHERE mov_id = mr.mov_id);
```

**Output:**
| mov_title           | max_stars |
|---------------------|-----------|
| aliens              | 8.4       |
| american beauty     | 7.0       |
| annie hall          | 8.1       |
| avatar              | 7.3       |
| beyond the sea      | 6.7       |
| blade runner        | 8.2       |
| boogie nights       | 3.0       |
| chinatown           | null      |
| donnie darko        | 8.1       |
| good will hunting   | 4.0       |
| lawrence of arabia  | 8.3       |
| princess mononoke   | 8.4       |
| seven samurai       | 7.7       |
| slumdog millionaire | 8.0       |
| the innocents       | 7.9       |
| the usual suspects  | 8.6       |
| titanic             | 7.7       |
| trainspotting       | null      |
| vertigo             | 8.4       |

---

## Query 22
**Question:** Write a SQL query to find all reviewers who rated the movie 'american beauty'. Return reviewer name.

**Query:**
```sql
SELECT rev_name FROM movie_reviewer 
WHERE rev_id IN (
    SELECT rev_id FROM movie_rating 
    WHERE mov_id = (
        SELECT mov_id FROM movie WHERE mov_title = 'american beauty'
    )
);
```

---

## Query 23
**Question:** Write a SQL query to find the movies that have not been reviewed by any reviewer body other than 'paul monks'. Return movie title.

**Query:**
```sql
SELECT mov_title FROM movie 
WHERE mov_id NOT IN (
    SELECT mov_id FROM movie_rating 
    WHERE rev_id IN (
        SELECT rev_id FROM movie_reviewer WHERE rev_name <> 'paul monks'
    )
);
```

**Output:**
| mov_title                |
|--------------------------|
| the deer hunter          |
| amadeus                  |
| blade runner             |
| eyes wide shut           |
| boogie nights            |
| princess mononoke        |
| the shawshank redemption |
| deliverance              |
| the prestige             |
| spirited away            |
| back to the future       |
| braveheart               |

---

## Query 24
**Question:** Write a SQL query to find the movies with the lowest ratings. Return reviewer name, movie title, and number of stars for those movies.

**Query:**
```sql
SELECT 
    (SELECT rev_name FROM movie_reviewer WHERE rev_id = mr.rev_id) AS rev_name,
    (SELECT mov_title FROM movie WHERE mov_id = mr.mov_id) AS mov_title,
    mr.rev_stars 
FROM movie_rating mr 
WHERE mr.rev_stars = (SELECT MIN(rev_stars) FROM movie_rating);
```

**Output:**
| rev_name   | mov_title     | rev_stars |
|------------|---------------|-----------|
| paul monks | boogie nights | 3.0       |

---

## Query 25
**Question:** Write a SQL query to find the movies directed by 'james cameron'. Return movie title.

**Query:**
```sql
SELECT m.mov_title 
FROM movie m 
JOIN movie_direction md ON m.mov_id = md.mov_id 
JOIN director d ON md.dir_id = d.dir_id
WHERE d.dir_fname = 'james' AND d.dir_lname = 'cameron';
```

**Output:**
| mov_title |
|-----------|
| titanic   |
| aliens    |

---

## Query 26
**Question:** Write a query in SQL to find the movies in which one or more actors appeared in more than one film.

**Query:**
```sql
SELECT DISTINCT m.mov_title 
FROM movie m 
JOIN movie_cast mc ON m.mov_id = mc.mov_id 
WHERE mc.act_id IN (
    SELECT act_id 
    FROM movie_cast 
    GROUP BY act_id 
    HAVING COUNT(mov_id) > 1
);
```

**Output:**
| mov_title       |
|-----------------|
| american beauty |
| beyond the sea  |

---

## Query 27
**Question:** Write a SQL query to find all reviewers whose ratings contain a null value. Return reviewer name.

**Query:**
```sql
SELECT DISTINCT r.rev_name 
FROM movie_reviewer r 
JOIN movie_rating mr ON r.rev_id = r.rev_id 
WHERE mr.rev_stars IS NULL;
```

**Output:**
| rev_name           |
|--------------------|
| righty sock        |
| jack malvern       |
| flagrant baronessa |
| alec shaw          |
| null               |
| victor woeltjen    |
| simon wright       |
| neal wruck         |
| paul monks         |
| mike salvati       |
| wesley s. walker   |
| sasha goldshtein   |
| josh cates         |
| krug stillo        |
| scott lebrun       |
| hannah steele      |
| vincent cadena     |
| brandt sponseller  |
| richard adams      |

---

## Query 28
**Question:** Write a SQL query to find out who was cast in the movie 'annie hall'. Return actor first name, last name and role.

**Query:**
```sql
SELECT a.act_fname, a.act_lname, mc.role 
FROM actor a 
JOIN movie_cast mc ON a.act_id = mc.act_id
WHERE mc.mov_id = (SELECT mov_id FROM movie WHERE mov_title = 'annie hall');
```

**Output:**
| act_fname | act_lname | role        |
|-----------|-----------|-------------|
| woody     | allen     | alvy singer |

---

## Query 29
**Question:** Write a SQL query to find the director who directed a movie that featured a role in 'eyes wide shut'. Return director first name, last name and movie title.

**Query:**
```sql
SELECT d.dir_fname, d.dir_lname, m.mov_title 
FROM director d 
JOIN movie_direction md ON d.dir_id = md.dir_id 
JOIN movie m ON md.mov_id = m.mov_id 
WHERE m.mov_id = (SELECT mov_id FROM movie WHERE mov_title = 'eyes wide shut');
```

**Output:**
| dir_fname | dir_lname | mov_title      |
|-----------|-----------|----------------|
| stanley   | kubrick   | eyes wide shut |

---

## Query 30
**Question:** Write a SQL query to find the director of a movie that cast a role as sean maguire. Return director first name, last name and movie title.

**Query:**
```sql
SELECT d.dir_fname, d.dir_lname, m.mov_title 
FROM director d 
JOIN movie_direction md ON d.dir_id = md.dir_id 
JOIN movie m ON md.mov_id = m.mov_id 
JOIN movie_cast mc ON m.mov_id = mc.mov_id 
WHERE mc.role = 'sean maguire';
```

**Output:**
| dir_fname | dir_lname | mov_title         |
|-----------|-----------|-------------------|
| gus       | van sant  | good will hunting |

---




## Query 31
**Question:** Write a SQL query to find out which actors have not appeared in any movies between 1990 and 2000 (begin and end values are included.). Return actor first name, last name, movie title and release year.

**Query:**
```sql
SELECT a.act_fname, a.act_lname, m.mov_title, m.mov_year 
FROM actor a 
JOIN movie_cast mc ON a.act_id = mc.act_id 
JOIN movie m ON mc.mov_id = m.mov_id 
WHERE a.act_id NOT IN (
    SELECT mc2.act_id
    FROM movie_cast mc2 
    JOIN movie m2 ON mc2.mov_id = m2.mov_id 
    WHERE m2.mov_year BETWEEN 1990 AND 2000
);
```

**Output:**
| act_fname | act_lname  | mov_title           | mov_year |
|-----------|------------|---------------------|----------|
| james     | stewart    | vertigo             | 1958     |
| deborah   | kerr       | the innocents       | 1961     |
| peter     | otoole     | lawrence of arabia  | 1962     |
| robert    | de niro    | the deer hunter     | 1978     |
| f. murray | abraham    | amadeus             | 1984     |
| harrison  | ford       | blade runner        | 1982     |
| woody     | allen      | annie hall          | 1977     |
| jon       | voight     | deliverance         | 1972     |
| maggie    | gyllenhaal | donnie darko        | 2001     |
| dev       | patel      | slumdog millionaire | 2008     |
| sigourney | weaver     | aliens              | 1986     |
| jack      | nicholson  | chinatown           | 1974     |
| christian | bale       | chinatown           | 1974     |

---

## Query 32
**Question:** Write a SQL query to find the directors who have directed films in a variety of genres. Group the result set on director first name, last name and generic title. Sort the result-set in ascending order by director first name and last name. Return director first name, last name and number of genres movies.

**Query:**
```sql
SELECT d.dir_fname, d.dir_lname, COUNT(DISTINCT g.gen_id) AS num_genres 
FROM director d, movie_direction md, movie_genres mg, genres g 
WHERE d.dir_id = md.dir_id 
AND md.mov_id = mg.mov_id 
AND mg.gen_id = g.gen_id 
GROUP BY d.dir_fname, d.dir_lname 
ORDER BY d.dir_fname ASC, d.dir_lname ASC;
```

**Output:**
| dir_fname | dir_lname | num_genres |
|-----------|-----------|------------|
| alfred    | hitchcock | 1          |
| bryan     | singer    | 1          |
| danny     | boyle     | 1          |
| david     | lean      | 1          |
| frank     | darabont  | 1          |
| hayao     | miyazaki  | 1          |
| jack      | clayton   | 1          |
| james     | cameron   | 1          |
| john      | boorman   | 1          |
| kevin     | spacey    | 1          |
| michael   | cimino    | 1          |
| ridley    | scott     | 1          |
| sam       | mendes    | 1          |
| stanley   | kubrick   | 1          |
| woody     | allen     | 1          |

---

## Query 33
**Question:** Write a SQL query to find the movies with year and genres. Return movie title, movie year and generic title.

**Query:**
```sql
SELECT m.mov_title, m.mov_year, g.gen_title 
FROM movie m 
JOIN movie_genres mg ON m.mov_id = mg.mov_id 
JOIN genres g ON mg.gen_id = g.gen_id;
```

**Output:**
| mov_title                | mov_year | gen_title |
|--------------------------|----------|-----------|
| aliens                   | 1986     | action    |
| deliverance              | 1972     | adventure |
| lawrence of arabia       | 1962     | adventure |
| princess mononoke        | 1997     | animation |
| annie hall               | 1977     | comedy    |
| the usual suspects       | 1995     | crime     |
| the shawshank redemption | 1994     | crime     |
| spirited away            | 2001     | drama     |
| braveheart               | 1995     | drama     |
| trainspotting            | 1996     | drama     |
| slumdog millionaire      | 2008     | drama     |
| the innocents            | 1961     | horror    |
| beyond the sea           | 2004     | music     |
| eyes wide shut           | 1999     | mystery   |
| back to the future       | 1985     | mystery   |
| vertigo                  | 1958     | mystery   |
| american beauty          | 1999     | romance   |
| blade runner             | 1982     | thriller  |
| the deer hunter          | 1978     | war       |

---

## Query 34
**Question:** Write a SQL query to find all the movies with year, genres, and name of the director.

**Query:**
```sql
SELECT m.mov_title, m.mov_year, g.gen_title, d.dir_fname, d.dir_lname
FROM movie m
JOIN movie_genres mg ON m.mov_id = mg.mov_id
JOIN genres g ON mg.gen_id = g.gen_id
JOIN movie_direction md ON m.mov_id = md.mov_id
JOIN director d ON md.dir_id = d.dir_id;
```

**Output:**
| mov_title                | mov_year | gen_title | dir_fname | dir_lname |
|--------------------------|----------|-----------|-----------|-----------|
| aliens                   | 1986     | action    | james     | cameron   |
| deliverance              | 1972     | adventure | john      | boorman   |
| lawrence of arabia       | 1962     | adventure | david     | lean      |
| princess mononoke        | 1997     | animation | hayao     | miyazaki  |
| annie hall               | 1977     | comedy    | woody     | allen     |
| the usual suspects       | 1995     | crime     | bryan     | singer    |
| the shawshank redemption | 1994     | crime     | frank     | darabont  |
| trainspotting            | 1996     | drama     | danny     | boyle     |
| slumdog millionaire      | 2008     | drama     | danny     | boyle     |
| the innocents            | 1961     | horror    | jack      | clayton   |
| beyond the sea           | 2004     | music     | kevin     | spacey    |
| eyes wide shut           | 1999     | mystery   | stanley   | kubrick   |
| vertigo                  | 1958     | mystery   | alfred    | hitchcock |
| american beauty          | 1999     | romance   | sam       | mendes    |
| blade runner             | 1982     | thriller  | ridley    | scott     |
| the deer hunter          | 1978     | war       | michael   | cimino    |

---

## Query 35
**Question:** Write a SQL query to find the movies released before 1st January 1989. Sort the result-set in descending order by date of release. Return movie title, release year, date of release, duration, and first and last name of the director.

**Query:**
```sql
SELECT m.mov_title, m.mov_year, m.mov_dt_rel, m.mov_time, d.dir_fname, d.dir_lname 
FROM movie m
JOIN movie_direction md ON m.mov_id = md.mov_id 
JOIN director d ON md.dir_id = d.dir_id 
WHERE m.mov_dt_rel < '1989-01-01' 
ORDER BY m.mov_dt_rel DESC;
```

**Output:**
| mov_title          | mov_year | mov_dt_rel | mov_time | dir_fname | dir_lname |
|--------------------|----------|------------|----------|-----------|-----------|
| aliens             | 1986     | 1986-08-29 | 137      | james     | cameron   |
| amadeus            | 1984     | 1985-01-07 | 160      | milos     | forman    |
| deliverance        | 1972     | 1982-10-05 | 109      | john      | boorman   |
| blade runner       | 1982     | 1982-09-09 | 117      | ridley    | scott     |
| the deer hunter    | 1978     | 1979-03-08 | 183      | michael   | cimino    |
| annie hall         | 1977     | 1977-04-20 | 93       | woody     | allen     |
| chinatown          | 1974     | 1974-08-09 | 130      | roman     | polanski  |
| lawrence of arabia | 1962     | 1962-12-11 | 216      | david     | lean      |
| the innocents      | 1961     | 1962-02-19 | 100      | jack      | clayton   |
| vertigo            | 1958     | 1958-08-24 | 128      | alfred    | hitchcock |

---

## Query 36
**Question:** Write a SQL query to calculate the average movie length and count the number of movies in each genre. Return genre title, average time and number of movies for each genre.

**Query:**
```sql
SELECT g.gen_title, AVG(m.mov_time) AS avg_time, COUNT(*) AS movie_count 
FROM genres g 
JOIN movie_genres mg ON g.gen_id = mg.gen_id 
JOIN movie m ON mg.mov_id = m.mov_id 
GROUP BY g.gen_title;
```

**Output:**
| gen_title | avg_time | movie_count |
|-----------|----------|-------------|
| action    | 137.0000 | 1           |
| adventure | 162.5000 | 2           |
| animation | 134.0000 | 1           |
| comedy    | 93.0000  | 1           |
| crime     | 124.0000 | 2           |
| drama     | 129.2500 | 4           |
| horror    | 100.0000 | 1           |
| music     | 118.0000 | 1           |
| mystery   | 134.3333 | 3           |
| romance   | 122.0000 | 1           |
| thriller  | 117.0000 | 1           |
| war       | 183.0000 | 1           |

---

## Query 37
**Question:** Write a SQL query to find movies with the shortest duration. Return movie title, movie year, director first name, last name, actor first name, last name and role.

**Query:**
```sql
SELECT m.mov_title, m.mov_year, d.dir_fname, d.dir_lname, a.act_fname, a.act_lname, mc.role
FROM movie m 
JOIN movie_direction md ON m.mov_id = md.mov_id 
JOIN director d ON md.dir_id = d.dir_id
JOIN movie_cast mc ON m.mov_id = mc.mov_id 
JOIN actor a ON mc.act_id = a.act_id 
WHERE m.mov_time = (SELECT MIN(mov_time) FROM movie);
```

**Output:**
| mov_title  | mov_year | dir_fname | dir_lname | act_fname | act_lname | role        |
|------------|----------|-----------|-----------|-----------|-----------|-------------|
| annie hall | 1977     | woody     | allen     | woody     | allen     | alvy singer |

---

## Query 38
**Question:** Write a SQL query to find the years in which a movie received a rating of 3 or 4. Sort the result in increasing order on movie year.

**Query:**
```sql
SELECT DISTINCT m.mov_year 
FROM movie m 
JOIN movie_rating mr ON m.mov_id = mr.mov_id 
WHERE mr.rev_stars IN (3, 4) 
ORDER BY m.mov_year;
```

**Output:**
| mov_year |
|----------|
| 1997     |

---

## Query 39
**Question:** Write a SQL query to get the reviewer name, movie title, and stars in an order that reviewer name will come first, then by movie title, and lastly by number of stars.

**Query:**
```sql
SELECT r.rev_name, m.mov_title, mr.rev_stars 
FROM movie_rating mr 
JOIN movie_reviewer r ON mr.rev_id = r.rev_id 
JOIN movie m ON mr.mov_id = m.mov_id 
ORDER BY r.rev_name, m.mov_title, mr.rev_stars;
```

**Output:**
| rev_name           | mov_title           | rev_stars |
|--------------------|---------------------|-----------|
| null               | blade runner        | 8.2       |
| null               | princess mononoke   | 8.4       |
| brandt sponseller  | aliens              | 8.4       |
| flagrant baronessa | lawrence of arabia  | 8.3       |
| hannah steele      | donnie darko        | 8.1       |
| jack malvern       | the innocents       | 7.9       |
| josh cates         | good will hunting   | 4.0       |
| krug stillo        | seven samurai       | 7.7       |
| mike salvati       | annie hall          | 8.1       |
| neal wruck         | chinatown           | null      |
| paul monks         | boogie nights       | 3.0       |
| richard adams      | beyond the sea      | 6.7       |
| righty sock        | titanic             | 7.7       |
| righty sock        | vertigo             | 8.4       |
| sasha goldshtein   | american beauty     | 7.0       |
| scott lebrun       | trainspotting       | null      |
| simon wright       | the usual suspects  | 8.6       |
| victor woeltjen    | avatar              | 7.3       |
| vincent cadena     | slumdog millionaire | 8.0       |

---

## Query 40
**Question:** Write a SQL query to find those movies that have at least one rating and received the most stars. Sort the result-set on movie title. Return movie title and maximum review stars.

**Query:**
```sql
SELECT m.mov_title, MAX(mr.rev_stars) AS max_stars 
FROM movie m 
JOIN movie_rating mr ON m.mov_id = mr.mov_id 
GROUP BY m.mov_title 
HAVING MAX(mr.rev_stars) = (SELECT MAX(rev_stars) FROM movie_rating)
ORDER BY m.mov_title;
```

**Output:**
| mov_title          | max_stars |
|--------------------|-----------|
| the usual suspects | 8.6       |

---

## Query 41
**Question:** Write a SQL query to find out which movies have received ratings. Return movie title, director first name, director last name and review stars.

**Query:**
```sql
SELECT m.mov_title, d.dir_fname, d.dir_lname, mr.rev_stars 
FROM movie m 
JOIN movie_rating mr ON m.mov_id = mr.mov_id 
JOIN movie_direction md ON m.mov_id = md.mov_id 
JOIN director d ON md.dir_id = d.dir_id;
```

**Output:**
| mov_title           | dir_fname | dir_lname       | rev_stars |
|---------------------|-----------|-----------------|-----------|
| vertigo             | alfred    | hitchcock       | 8.4       |
| the innocents       | jack      | clayton         | 7.9       |
| lawrence of arabia  | david     | lean            | 8.3       |
| blade runner        | ridley    | scott           | 8.2       |
| the usual suspects  | bryan     | singer          | 8.6       |
| chinatown           | roman     | polanski        | null      |
| boogie nights       | paul      | thomas anderson | 3.0       |
| annie hall          | woody     | allen           | 8.1       |
| princess mononoke   | hayao     | miyazaki        | 8.4       |
| american beauty     | sam       | mendes          | 7.0       |
| titanic             | james     | cameron         | 7.7       |
| good will hunting   | gus       | van sant        | 4.0       |
| trainspotting       | danny     | boyle           | null      |
| donnie darko        | richard   | kelly           | 8.1       |
| slumdog millionaire | danny     | boyle           | 8.0       |
| aliens              | james     | cameron         | 8.4       |
| beyond the sea      | kevin     | spacey          | 6.7       |

---

## Query 42
**Question:** Write a SQL query to find movies in which one or more actors have acted in more than one film. Return movie title, actor first and last name, and the role.

**Query:**
```sql
SELECT m.mov_title, a.act_fname, a.act_lname, mc.role 
FROM movie m 
JOIN movie_cast mc ON m.mov_id = mc.mov_id 
JOIN actor a ON mc.act_id = a.act_id 
WHERE a.act_id IN (
    SELECT act_id 
    FROM movie_cast 
    GROUP BY act_id 
    HAVING COUNT(mov_id) > 1
);
```

**Output:**
| mov_title       | act_fname | act_lname | role           |
|-----------------|-----------|-----------|----------------|
| american beauty | kevin     | spacey    | lester burnham |
| beyond the sea  | kevin     | spacey    | bobby darin    |

---

## Query 43
**Question:** Write a SQL query to find the actor whose first name is 'claire' and last name is 'danes'. Return director first name, last name, movie title, actor first name and last name, role.

**Query:**
```sql
SELECT d.dir_fname, d.dir_lname, m.mov_title, a.act_fname, a.act_lname, mc.role
FROM actor a 
JOIN movie_cast mc ON a.act_id = mc.act_id 
JOIN movie m ON mc.mov_id = m.mov_id 
JOIN movie_direction md ON m.mov_id = md.mov_id 
JOIN director d ON md.dir_id = d.dir_id 
WHERE a.act_fname = 'claire' AND a.act_lname = 'danes';
```

**Output:**
| dir_fname | dir_lname | mov_title         | act_fname | act_lname | role |
|-----------|-----------|-------------------|-----------|-----------|------|
| hayao     | miyazaki  | princess mononoke | claire    | danes     | san  |

---

## Query 44
**Question:** Write a SQL query to find for actors whose films have been directed by them. Return actor first name, last name, movie title and role.

**Query:**
```sql
SELECT a.act_fname, a.act_lname, m.mov_title, mc.role
FROM actor a
JOIN movie_cast mc ON a.act_id = mc.act_id
JOIN movie m ON mc.mov_id = m.mov_id
JOIN movie_direction md ON m.mov_id = md.mov_id
JOIN director d ON md.dir_id = d.dir_id
WHERE a.act_fname = d.dir_fname AND a.act_lname = d.dir_lname;
```

**Output:**
| act_fname | act_lname | mov_title      | role        |
|-----------|-----------|----------------|-------------|
| woody     | allen     | annie hall     | alvy singer |
| kevin     | spacey    | beyond the sea | bobby darin |

---

## Query 45
**Question:** Write a SQL query to find the cast list of the movie 'chinatown'. Return first name, last name.

**Query:**
```sql
SELECT a.act_fname, a.act_lname
FROM actor a
JOIN movie_cast mc ON a.act_id = mc.act_id
WHERE mc.mov_id = (
    SELECT mov_id FROM movie WHERE mov_title = 'chinatown'
);
```

**Output:**
| act_fname | act_lname |
|-----------|-----------|
| jack      | nicholson |
| christian | bale      |

---

## Query 46
**Question:** Write a SQL query to find those movies where actor's first name is 'harrison' and last name is 'ford'. Return movie title.

**Query:**
```sql
SELECT m.mov_title
FROM movie m
JOIN movie_cast mc ON m.mov_id = mc.mov_id
JOIN actor a ON mc.act_id = a.act_id
WHERE a.act_fname = 'harrison' AND a.act_lname = 'ford';
```

**Output:**
| mov_title    |
|--------------|
| blade runner |

---

## Query 47
**Question:** Write a SQL query to find the highest-rated movies. Return movie title, movie year, review stars and releasing country.

**Query:**
```sql
SELECT m.mov_title, m.mov_year, mr.rev_stars, m.mov_rel_country
FROM movie m
JOIN movie_rating mr ON m.mov_id = mr.mov_id
WHERE mr.rev_stars = (
    SELECT MAX(rev_stars) FROM movie_rating
);
```

**Output:**
| mov_title          | mov_year | rev_stars | mov_rel_country |
|--------------------|----------|-----------|-----------------|
| the usual suspects | 1995     | 8.6       | uk              |

---

## Query 48
**Question:** Write a SQL query to find the highest-rated 'mystery movies'. Return the title, year, and rating.

**Query:**
```sql
SELECT m.mov_title, m.mov_year, mr.rev_stars
FROM movie m
JOIN movie_rating mr ON m.mov_id = mr.mov_id
JOIN movie_genres mg ON m.mov_id = mg.mov_id
JOIN genres g ON mg.gen_id = g.gen_id
WHERE g.gen_title = 'mystery'
  AND mr.rev_stars = (
    SELECT MAX(mr2.rev_stars)
    FROM movie_rating mr2
    JOIN movie_genres mg2 ON mr2.mov_id = mg2.mov_id
    JOIN genres g2 ON mg2.gen_id = g2.gen_id
    WHERE g2.gen_title = 'mystery'
);
```

**Output:**
| mov_title | mov_year | rev_stars |
|-----------|----------|-----------|
| vertigo   | 1958     | 8.4       |

---

## Query 49
**Question:** Write a SQL query to find the years when most of the 'mystery movies' produced. Count the number of generic title and compute their average rating. Group the result set on movie release year, generic title. Return movie year, generic title, number of generic title and average rating.

**Query:**
```sql
SELECT m.mov_year, g.gen_title,
       COUNT(*) AS total_movies,
       AVG(mr.rev_stars) AS avg_rating
FROM movie m
JOIN movie_genres mg ON m.mov_id = mg.mov_id
JOIN genres g ON mg.gen_id = g.gen_id
JOIN movie_rating mr ON m.mov_id = mr.mov_id
WHERE g.gen_title = 'mystery'
GROUP BY m.mov_year, g.gen_title;
```

**Output:**
| mov_year | gen_title | total_movies | avg_rating |
|----------|-----------|--------------|------------|
| 1958     | mystery   | 1            | 8.40000    |

---

## Query 50
**Question:** Write a query in SQL to generate a report, which contain the fields movie title, name of the female actor, year of the movie, role, movie genres, the director, date of release, and rating of that movie.

**Query:**
```sql
SELECT m.mov_title, a.act_fname, a.act_lname, m.mov_year, mc.role, g.gen_title, 
       d.dir_fname, d.dir_lname, m.mov_dt_rel, mr.rev_stars
FROM movie m
JOIN movie_cast mc ON m.mov_id = mc.mov_id
JOIN actor a ON mc.act_id = a.act_id
JOIN movie_genres mg ON m.mov_id = mg.mov_id
JOIN genres g ON mg.gen_id = g.gen_id
JOIN movie_direction md ON m.mov_id = md.mov_id
JOIN director d ON md.dir_id = d.dir_id
JOIN movie_rating mr ON m.mov_id = mr.mov_id
WHERE a.act_gender = 'f';
```

**Output:**
| mov_title         | act_fname | act_lname | mov_year | role         | gen_title | dir_fname | dir_lname | mov_dt_rel | rev_stars |
|-------------------|-----------|-----------|----------|--------------|-----------|-----------|-----------|------------|-----------|
| the innocents     | deborah   | kerr      | 1961     | miss giddens | horror    | jack      | clayton   | 1962-02-19 | 7.9       |
| princess mononoke | claire    | danes     | 1997     | san          | animation | hayao     | miyazaki  | 2001-10-19 | 8.4       |
| aliens            | sigourney | weaver    | 1986     | ripley       | action    | james     | cameron   | 1986-08-29 | 8.4       |


