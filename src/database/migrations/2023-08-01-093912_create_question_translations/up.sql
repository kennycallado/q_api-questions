CREATE TABLE IF NOT EXISTS question_translations ();

ALTER TABLE question_translations
  ADD COLUMN id SERIAL PRIMARY KEY,
  ADD COLUMN question_id INTEGER NOT NULL,
  ADD COLUMN locale VARCHAR(5) NOT NULL,
  ADD COLUMN question VARCHAR NOT NULL,
  ADD CONSTRAINT fk_qt_question FOREIGN KEY (question_id) REFERENCES questions (id) ON DELETE CASCADE;

INSERT INTO question_translations (question_id, locale, question) VALUES
  (1, 'en', 'How often do you feel confident in your ability to achieve your goals?'),
  (1, 'es', '¿Con qué frecuencia te sientes seguro de tu capacidad para alcanzar tus metas?'),
  (2, 'en', 'How often do you feel capable of successfully overcoming challenges?'),
  (2, 'es', '¿Con qué frecuencia te sientes capaz de superar con éxito los desafíos?'),
  (3, 'en', 'How often do you feel you are able to make wise decisions?'),
  (3, 'es', '¿Con qué frecuencia sientes que puedes tomar decisiones sabias?'),
  (4, 'en', 'How often do you feel you have the strength to stay resilient in difficult situations?'),
  (4, 'es', '¿Con qué frecuencia sientes que tienes la fuerza para mantener la resiliencia en situaciones difíciles?'),
  (5, 'en', 'How often do you feel you have the power to make positive changes in your life?'),
  (5, 'es', '¿Con qué frecuencia sientes que tienes el poder para hacer cambios positivos en tu vida?');
