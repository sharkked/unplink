<template>
  <main>
    <div class="plink-cat" />
    <p>
      i really don't know what it is i'm doing here, i just know that i need a
      link shortener and i don't want to pay for it.
    </p>
  </main>
</template>

<style>
.plink-cat {
  margin: 0 auto;

  &,
  &:after {
    width: 318px;
    height: 128px;
    background-image: url("https://c.tenor.com/y1QFa-1vyKYAAAAd/tenor.gif");
    background-size: contain;
    border-radius: var(--radius);
  }

  &:after {
    content: "";
    position: absolute;
    inset: 0;
    z-index: -1;
    opacity: 40%;
    filter: blur(15px) saturate(1.5);
  }
}
</style>
