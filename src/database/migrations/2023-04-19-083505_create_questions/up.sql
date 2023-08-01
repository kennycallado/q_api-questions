CREATE TABLE IF NOT EXISTS questions ();

ALTER TABLE questions
  ADD COLUMN IF NOT EXISTS id SERIAL PRIMARY KEY,
  ADD COLUMN IF NOT EXISTS question_type VARCHAR(10) NOT NULL;
  -- ADD COLUMN IF NOT EXISTS question VARCHAR NOT NULL;

ALTER TABLE questions ADD CONSTRAINT questions_question_type_check CHECK
  (question_type IN ('range', 'radio', 'checkbox', 'input'));

INSERT INTO questions (question_type) VALUES
  ('range'),
  ('range'),
  ('range'),
  ('range'),
  ('range')
  ;

-- SELECT diesel_manage_updated_at('questions');
-- INSERT INTO questions (question_type, question) VALUES
--   ('range', 'How often do you feel confident in your ability to achieve your goals?'),
--   ('range', 'How often do you feel capable of successfully overcoming challenges?'),
--   ('range', 'How often do you feel you are able to make wise decisions?'),
--   ('range', 'How often do you feel you have the strength to stay resilient in difficult situations?'),
--   ('range', 'How often do you feel you have the power to make positive changes in your life?')
--   ;
