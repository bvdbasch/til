<?php
require_once 'Twig/Autoloader.php';
Twig_Autoloader::register();
$loader = new Twig_Loader_Filesystem('Templates');
$twig = new Twig_Environment($loader);
$template = $twig->loadTemplate('greeting.phtml');
?>

<!doctype html>
<html lang="en">
  <head>
    <meta charset="utf-8">
    <meta http-equiv="X-UA-Compatible" content="IE=edge">
    <title>First Flight</title>
    <link rel="stylesheet" href="style.css">
    <script src="script.js"></script>
  </head>
  <body>
      
      <?php
        $params = array('name' => 'Vexxie');
        $template->display($params);
      ?>
      
  </body>
</html>