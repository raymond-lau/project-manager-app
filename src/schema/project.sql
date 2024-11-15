
DROP TABLE IF EXISTS `project`;

CREATE TABLE `project` (
  `id` INTEGER PRIMARY KEY,
  `project_name` varchar(100) NULL,
  `company_name` varchar(100) NULL,
  `ip_address` varchar(100) NULL
);