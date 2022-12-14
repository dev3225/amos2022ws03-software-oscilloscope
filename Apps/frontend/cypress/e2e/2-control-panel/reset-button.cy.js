/// <reference types="cypress" />

describe("everything visible on front page", () => {
  beforeEach(() => {
    cy.visit("http://localhost:5173/");
  });

  it("click btn", () => {
    // We use the `cy.get()` to get the canvas element
    cy.get("#button-reset").click();
  });
});