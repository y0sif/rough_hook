#include "hooklenswindow.h"
#include <QVBoxLayout>
#include <QHBoxLayout>
#include <QLabel>
#include <QPushButton>
#include <QLineEdit>
#include <QFileDialog>
#include <QPixmap>
#include <QScrollArea>
#include <QMessageBox>
#include <QDir>
#include <QProcess>
#include <QStandardPaths>
#include <QFile>
#include <QTextStream>

HookLensWindow::HookLensWindow(QWidget* parent)
    : QWidget(parent)
{
    setWindowTitle("Hook Lens");
    
    // Set minimum size for the window
    setMinimumSize(800, 600);
    
    // Create the main layout
    m_mainLayout = new QVBoxLayout(this);
    
    // Create buttons layout
    QHBoxLayout* buttonLayout = new QHBoxLayout();
    
    // Create and add the "Add Photo" button
    m_addPhotoButton = new QPushButton("Add Photo", this);
    m_addPhotoButton->setMaximumWidth(120);
    buttonLayout->addWidget(m_addPhotoButton);
    
    // Create and add the "Send" button
    m_sendButton = new QPushButton("Send", this);
    m_sendButton->setMaximumWidth(120);
    m_sendButton->setEnabled(false); // Initially disabled
    buttonLayout->addWidget(m_sendButton);
    
    // Create and add the "Best Move" button
    m_bestMoveButton = new QPushButton("Best Move", this);
    m_bestMoveButton->setMaximumWidth(120);
    m_bestMoveButton->setEnabled(false); // Initially disabled
    buttonLayout->addWidget(m_bestMoveButton);
    
    // Create and add the "Reset" button
    m_resetButton = new QPushButton("Reset", this);
    m_resetButton->setMaximumWidth(120);
    buttonLayout->addWidget(m_resetButton);
    
    // Add stretch to push buttons to the left
    buttonLayout->addStretch();
    
    m_mainLayout->addLayout(buttonLayout);
    
    // Create and add the path text field
    QLabel* pathLabel = new QLabel("Photo Path:", this);
    m_pathLineEdit = new QLineEdit(this);
    m_pathLineEdit->setReadOnly(true);
    m_pathLineEdit->setPlaceholderText("No photo selected...");
    
    m_mainLayout->addWidget(pathLabel);
    m_mainLayout->addWidget(m_pathLineEdit);
    
    // Create and add the result text field
    QLabel* resultLabel = new QLabel("Result from Rust:", this);
    m_resultLineEdit = new QLineEdit(this);
    m_resultLineEdit->setReadOnly(true);
    m_resultLineEdit->setPlaceholderText("No result yet...");
    
    m_mainLayout->addWidget(resultLabel);
    m_mainLayout->addWidget(m_resultLineEdit);
    
    // Create best move fields layout
    QHBoxLayout* bestMoveLayout = new QHBoxLayout();
    
    // Create and add the "From Square" text field
    QLabel* fromLabel = new QLabel("From Square:", this);
    m_fromSquareEdit = new QLineEdit(this);
    m_fromSquareEdit->setReadOnly(true);
    m_fromSquareEdit->setPlaceholderText("e.g., g4");
    m_fromSquareEdit->setMaximumWidth(100);
    
    bestMoveLayout->addWidget(fromLabel);
    bestMoveLayout->addWidget(m_fromSquareEdit);
    
    // Create and add the "To Square" text field
    QLabel* toLabel = new QLabel("To Square:", this);
    m_toSquareEdit = new QLineEdit(this);
    m_toSquareEdit->setReadOnly(true);
    m_toSquareEdit->setPlaceholderText("e.g., f6");
    m_toSquareEdit->setMaximumWidth(100);
    
    bestMoveLayout->addWidget(toLabel);
    bestMoveLayout->addWidget(m_toSquareEdit);
    
    // Add stretch to push fields to the left
    bestMoveLayout->addStretch();
    
    m_mainLayout->addLayout(bestMoveLayout);
    
    // Create a scroll area for the image
    QScrollArea* scrollArea = new QScrollArea(this);
    scrollArea->setWidgetResizable(true);
    scrollArea->setMinimumHeight(400);
    
    // Create the image label
    m_imageLabel = new QLabel(this);
    m_imageLabel->setAlignment(Qt::AlignCenter);
    m_imageLabel->setText("No image selected");
    m_imageLabel->setStyleSheet("QLabel { border: 2px dashed #aaa; color: #999; }");
    m_imageLabel->setMinimumSize(400, 300);
    
    scrollArea->setWidget(m_imageLabel);
    m_mainLayout->addWidget(scrollArea);
    
    // Connect the button signals to the slots
    connect(m_addPhotoButton, &QPushButton::clicked, this, &HookLensWindow::onAddPhotoClicked);
    connect(m_sendButton, &QPushButton::clicked, this, &HookLensWindow::onSendClicked);
    connect(m_bestMoveButton, &QPushButton::clicked, this, &HookLensWindow::onBestMoveClicked);
    connect(m_resetButton, &QPushButton::clicked, this, &HookLensWindow::onResetClicked);
    
    // Set the layout to the widget
    setLayout(m_mainLayout);
    
    // Make it a proper window (not embedded)
    setWindowFlags(Qt::Window);
}

void HookLensWindow::onAddPhotoClicked()
{
    // Open file dialog to select an image
    QString fileName = QFileDialog::getOpenFileName(
        this,
        "Select Photo",
        QDir::homePath(),
        "Image Files (*.png *.jpg *.jpeg *.bmp *.gif *.tiff);;All Files (*.*)"
    );
    
    if (!fileName.isEmpty())
    {
        // Display the file path in the text field
        m_pathLineEdit->setText(fileName);
        
        // Load and display the image
        QPixmap pixmap(fileName);
        if (!pixmap.isNull())
        {
            // Scale the image to fit the label while keeping aspect ratio
            QPixmap scaledPixmap = pixmap.scaled(m_imageLabel->size(), Qt::KeepAspectRatio, Qt::SmoothTransformation);
            m_imageLabel->setPixmap(scaledPixmap);
            m_imageLabel->setText(""); // Clear the "No image selected" text
            
            // Enable the Send button now that we have a valid image
            m_sendButton->setEnabled(true);
        }
        else
        {
            // Show error message if image couldn't be loaded
            QMessageBox::warning(this, "Error", "Could not load the selected image file.");
            m_pathLineEdit->clear();
            m_sendButton->setEnabled(false);
        }
    }
}

void HookLensWindow::onSendClicked()
{
    QString imagePath = m_pathLineEdit->text();
    
    if (imagePath.isEmpty())
    {
        QMessageBox::warning(this, "Error", "No image path to send.");
        return;
    }
    
    // Path to the existing Rust project
    QString rustProjectPath = "/home/sasa/My_Projects/Graduation_Project/rough_hook/hook_lens";
    
    // Clear the result field
    m_resultLineEdit->clear();
    m_resultLineEdit->setPlaceholderText("Processing...");
    
    // Try to run the Rust project with the image path as an argument
    QProcess* process = new QProcess(this);
    process->setWorkingDirectory(rustProjectPath);
    
    // Connect to handle the process completion
    connect(process, QOverload<int, QProcess::ExitStatus>::of(&QProcess::finished),
        [this, process, imagePath](int exitCode, QProcess::ExitStatus exitStatus) {
            if (exitStatus == QProcess::NormalExit && exitCode == 0)
            {
                // Read the output from the Rust program
                QString output = process->readAllStandardOutput().trimmed();
                
                // Display the result in the result text field
                if (!output.isEmpty())
                {
                    m_resultLineEdit->setText(output);
                    emit fenStringReady(output);
                    // Enable the Best Move button now that we have a FEN string
                    m_bestMoveButton->setEnabled(true);
                    QMessageBox::information(this, "Success", 
                        QString("Received result from Rust project:\n%1").arg(output));
                }
                else
                {
                    m_resultLineEdit->setText("No output received");
                    m_bestMoveButton->setEnabled(false);
                    QMessageBox::information(this, "Success", "Rust project executed successfully but no output received.");
                }
            }
            else
            {
                QString error = process->readAllStandardError();
                m_resultLineEdit->setText("Error occurred");
                m_resultLineEdit->setPlaceholderText("Error occurred");
                m_bestMoveButton->setEnabled(false);
                QMessageBox::warning(this, "Error", 
                    QString("Failed to run Rust project.\nError: %1").arg(error));
            }
            process->deleteLater();
        });
    
    // Start the Rust process
   

    // Clone the current environment
    QStringList env = QProcess::systemEnvironment();
    env << "RUSTFLAGS=-Awarnings";  // Suppress warnings
    process->setEnvironment(env);

    // Prepare arguments
    QStringList arguments;
    arguments << "run" << "--" << imagePath;

    // Start the process
    process->start("cargo", arguments);
}

void HookLensWindow::onBestMoveClicked()
{
    QString fenString = m_resultLineEdit->text();
    
    if (fenString.isEmpty())
    {
        QMessageBox::warning(this, "Error", "No FEN string available. Please send an image first.");
        return;
    }
    
    // Clear the best move fields
    m_fromSquareEdit->clear();
    m_toSquareEdit->clear();
    m_fromSquareEdit->setPlaceholderText("Processing...");
    m_toSquareEdit->setPlaceholderText("Processing...");
    
    // Path to the integration Rust project
    QString integrationProjectPath = "/home/sasa/My_Projects/Graduation_Project/rough_hook/integration";
    
    // Try to run the integration Rust project with the FEN string as an argument
    QProcess* process = new QProcess(this);
    process->setWorkingDirectory(integrationProjectPath);
    
    // Connect to handle the process completion
    connect(process, QOverload<int, QProcess::ExitStatus>::of(&QProcess::finished),
        [this, process, fenString](int exitCode, QProcess::ExitStatus exitStatus) {
            if (exitStatus == QProcess::NormalExit && exitCode == 0)
            {
                // Read the output from the Rust program
                QString output = process->readAllStandardOutput().trimmed();
                
                // Parse the output (expected format: "g4f6" where g4 is from and f6 is to)
                if (!output.isEmpty() && output.length() >= 4)
                {
                    QString fromSquare = output.left(2);
                    QString toSquare = output.mid(2, 2);
                    
                    m_fromSquareEdit->setText(fromSquare);
                    m_toSquareEdit->setText(toSquare);
                    
                    // Emit signal to notify main window about the best move
                    emit bestMoveReady(fromSquare, toSquare);
                    
                    QMessageBox::information(this, "Success", 
                        QString("Best move found: %1 -> %2").arg(fromSquare, toSquare));
                }
                else
                {
                    m_fromSquareEdit->setText("Error");
                    m_toSquareEdit->setText("Error");
                    m_fromSquareEdit->setPlaceholderText("Invalid output");
                    m_toSquareEdit->setPlaceholderText("Invalid output");
                    QMessageBox::warning(this, "Error", 
                        QString("Invalid output format from integration project: %1").arg(output));
                }
            }
            else
            {
                QString error = process->readAllStandardError();
                m_fromSquareEdit->setText("Error");
                m_toSquareEdit->setText("Error");
                m_fromSquareEdit->setPlaceholderText("Error occurred");
                m_toSquareEdit->setPlaceholderText("Error occurred");
                QMessageBox::warning(this, "Error", 
                    QString("Failed to run integration project.\nError: %1").arg(error));
            }
            process->deleteLater();
        });
    
    // Clone the current environment
    QStringList env = QProcess::systemEnvironment();
    env << "RUSTFLAGS=-Awarnings";  // Suppress warnings
    process->setEnvironment(env);

    // Prepare arguments
    QStringList arguments;
    arguments << "run" << "--" << fenString;

    // Start the process
    process->start("cargo", arguments);
}

void HookLensWindow::onResetClicked()
{
    // Clear all text fields
    m_pathLineEdit->clear();
    m_pathLineEdit->setPlaceholderText("No photo selected...");
    
    m_resultLineEdit->clear();
    m_resultLineEdit->setPlaceholderText("No result yet...");
    
    m_fromSquareEdit->clear();
    m_fromSquareEdit->setPlaceholderText("e.g., g4");
    
    m_toSquareEdit->clear();
    m_toSquareEdit->setPlaceholderText("e.g., f6");
    
    // Reset the image label to initial state
    m_imageLabel->clear();
    m_imageLabel->setText("No image selected");
    m_imageLabel->setStyleSheet("QLabel { border: 2px dashed #aaa; color: #999; }");
    
    // Reset all buttons to their initial state
    m_addPhotoButton->setEnabled(true);  // Always enabled
    m_sendButton->setEnabled(false);     // Disabled until image is loaded
    m_bestMoveButton->setEnabled(false); // Disabled until FEN is received
    
    // Show confirmation message
    QMessageBox::information(this, "Reset Complete", 
        "All fields have been cleared and the window has been reset to its initial state.\n\n"
        "You can now load a new image and start the pipeline again.");

    emit resetRequested();
}
